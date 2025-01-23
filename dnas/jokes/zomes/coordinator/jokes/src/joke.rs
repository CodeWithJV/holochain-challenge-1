use hdk::prelude::*;
use jokes_integrity::*;

// Add your Zome functions here!
#[hdk_extern]
pub fn create_joke(joke: Joke) -> ExternResult<Record> {
    let joke_hash = create_entry(&EntryTypes::Joke(joke.clone()))?;
    let record = get(joke_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Joke".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_joke_by_hash(original_joke_hash: AnyDhtHash) -> ExternResult<Option<Details>> {
    let Some(details) = get_details(original_joke_hash, GetOptions::default())? else {
        return Ok(None);
    };

    match details {
        Details::Record(details) => Ok(Some(Details::Record(details))),
        Details::Entry(details) => Ok(Some(Details::Entry(details))),
        _ => {
            Err(wasm_error!(WasmErrorInner::Guest(String::from("Malformed get details response"))))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateJokeInput {
    pub original_joke_hash: ActionHash,
    pub previous_joke_hash: ActionHash,
    pub updated_joke: Joke,
}

#[hdk_extern]
pub fn update_joke(input: UpdateJokeInput) -> ExternResult<Record> {
    let updated_joke_hash = update_entry(input.previous_joke_hash.clone(), &input.updated_joke)?;
    let record = get(updated_joke_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly updated Joke".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_joke(original_joke_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_joke_hash)
}
