pub mod joke;
pub use joke::*;
use hdi::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Joke(Joke),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    CreatorToJokes,
    JokeUpdates,
}

#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => {
            match store_entry {
                OpEntry::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Joke(joke) => {
                            validate_create_joke(
                                EntryCreationAction::Create(action),
                                joke,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::Joke(joke) => {
                            validate_create_joke(
                                EntryCreationAction::Update(action),
                                joke,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterUpdate(update_entry) => {
            match update_entry {
                OpUpdate::Entry { app_entry, action } => {
                    let original_action = must_get_action(
                            action.clone().original_action_address,
                        )?
                        .action()
                        .to_owned();
                    let original_create_action = match EntryCreationAction::try_from(
                        original_action,
                    ) {
                        Ok(action) => action,
                        Err(e) => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    format!(
                                        "Expected to get EntryCreationAction from Action: {e:?}"
                                    ),
                                ),
                            );
                        }
                    };
                    match app_entry {
                        EntryTypes::Joke(joke) => {
                            let original_app_entry = must_get_valid_record(
                                action.clone().original_action_address,
                            )?;
                            let original_joke = match Joke::try_from(
                                original_app_entry,
                            ) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(
                                        ValidateCallbackResult::Invalid(
                                            format!("Expected to get Joke from Record: {e:?}"),
                                        ),
                                    );
                                }
                            };
                            validate_update_joke(
                                action,
                                joke,
                                original_create_action,
                                original_joke,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterDelete(delete_entry) => {
            let original_action_hash = delete_entry.clone().action.deletes_address;
            let original_record = must_get_valid_record(original_action_hash)?;
            let original_record_action = original_record.action().clone();
            let original_action = match EntryCreationAction::try_from(
                original_record_action,
            ) {
                Ok(action) => action,
                Err(e) => {
                    return Ok(
                        ValidateCallbackResult::Invalid(
                            format!(
                                "Expected to get EntryCreationAction from Action: {e:?}"
                            ),
                        ),
                    );
                }
            };
            let app_entry_type = match original_action.entry_type() {
                EntryType::App(app_entry_type) => app_entry_type,
                _ => {
                    return Ok(ValidateCallbackResult::Valid);
                }
            };
            let entry = match original_record.entry().as_option() {
                Some(entry) => entry,
                None => {
                    return Ok(
                        ValidateCallbackResult::Invalid(
                            "Original record for a delete must contain an entry"
                                .to_string(),
                        ),
                    );
                }
            };
            let original_app_entry = match EntryTypes::deserialize_from_type(
                app_entry_type.zome_index,
                app_entry_type.entry_index,
                entry,
            )? {
                Some(app_entry) => app_entry,
                None => {
                    return Ok(
                        ValidateCallbackResult::Invalid(
                            "Original app entry must be one of the defined entry types for this zome"
                                .to_string(),
                        ),
                    );
                }
            };
            match original_app_entry {
                EntryTypes::Joke(original_joke) => {
                    validate_delete_joke(
                        delete_entry.clone().action,
                        original_action,
                        original_joke,
                    )
                }
            }
        }
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => {
            match link_type {
                LinkTypes::CreatorToJokes => {
                    validate_create_link_creator_to_jokes(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::JokeUpdates => {
                    validate_create_link_joke_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => {
            match link_type {
                LinkTypes::CreatorToJokes => {
                    validate_delete_link_creator_to_jokes(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::JokeUpdates => {
                    validate_delete_link_joke_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::StoreRecord(store_record) => {
            match store_record {
                OpRecord::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Joke(joke) => {
                            validate_create_joke(
                                EntryCreationAction::Create(action),
                                joke,
                            )
                        }
                    }
                }
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for an update must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match app_entry {
                        EntryTypes::Joke(joke) => {
                            let result = validate_create_joke(
                                EntryCreationAction::Update(action.clone()),
                                joke.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_joke: Option<Joke> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_joke = match original_joke {
                                    Some(joke) => joke,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_joke(
                                    action,
                                    joke,
                                    original_action,
                                    original_joke,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                OpRecord::DeleteEntry { original_action_hash, action, .. } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for a delete must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original record for a delete must contain an entry"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index,
                        app_entry_type.entry_index,
                        entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::Joke(original_joke) => {
                            validate_delete_joke(action, original_action, original_joke)
                        }
                    }
                }
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => {
                    match link_type {
                        LinkTypes::CreatorToJokes => {
                            validate_create_link_creator_to_jokes(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::JokeUpdates => {
                            validate_create_link_joke_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                    }
                }
                OpRecord::DeleteLink { original_action_hash, base_address, action } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "The action that a DeleteLink deletes must be a CreateLink"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index,
                        create_link.link_type,
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::CreatorToJokes => {
                            validate_delete_link_creator_to_jokes(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::JokeUpdates => {
                            validate_delete_link_joke_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterAgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateAgent { agent, action } => {
                    let previous_action = must_get_action(action.prev_action)?;
                    match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    }
}
