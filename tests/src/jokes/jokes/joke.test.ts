import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from '@holochain/tryorama';
import {
  NewEntryAction,
  ActionHash,
  Record,
  Link,
  CreateLink,
  DeleteLink,
  SignedActionHashed,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash
} from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createJoke, sampleJoke } from './common.js';

test('create Joke', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/demo.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Joke
    const record: Record = await createJoke(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read Joke', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/demo.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleJoke(alice.cells[0]);

    // Alice creates a Joke
    const record: Record = await createJoke(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created Joke
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_original_joke",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);

    // Bob gets the Creators for the new Joke
    let linksToCreators: Link[] = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_jokes_for_creator",
      payload: sample.creator
    });
    assert.equal(linksToCreators.length, 1);
    assert.deepEqual(linksToCreators[0].target, record.signed_action.hashed.hash);
  });
});

test('create and update Joke', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/demo.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Joke
    const record: Record = await createJoke(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the Joke
    let contentUpdate: any = await sampleJoke(alice.cells[0]);
    let updateInput = {
      original_joke_hash: originalActionHash,
      previous_joke_hash: originalActionHash,
      updated_joke: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "update_joke",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob gets the updated Joke
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_latest_joke",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the Joke again
    contentUpdate = await sampleJoke(alice.cells[0]);
    updateInput = { 
      original_joke_hash: originalActionHash,
      previous_joke_hash: updatedRecord.signed_action.hashed.hash,
      updated_joke: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "update_joke",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob gets the updated Joke
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_latest_joke",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);

    // Bob gets all the revisions for Joke
    const revisions: Record[] = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_all_revisions_for_joke",
      payload: originalActionHash,
    });
    assert.equal(revisions.length, 3);
    assert.deepEqual(contentUpdate, decode((revisions[2].entry as any).Present.entry) as any);
  });
});

test('create and delete Joke', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/demo.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleJoke(alice.cells[0]);

    // Alice creates a Joke
    const record: Record = await createJoke(alice.cells[0], sample);
    assert.ok(record);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the Creators for the new Joke
    let linksToCreators: Link[] = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_jokes_for_creator",
      payload: sample.creator
    });
    assert.equal(linksToCreators.length, 1);
    assert.deepEqual(linksToCreators[0].target, record.signed_action.hashed.hash);

    // Alice deletes the Joke
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "delete_joke",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob gets the oldest delete for the Joke
    const oldestDeleteForJoke: SignedActionHashed = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_oldest_delete_for_joke",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(oldestDeleteForJoke);
        
    // Bob gets the deletions for the Joke
    const deletesForJoke: SignedActionHashed[] = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_all_deletes_for_joke",
      payload: record.signed_action.hashed.hash,
    });
    assert.equal(deletesForJoke.length, 1);

    // Bob gets the Creators for the Joke again
    linksToCreators = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_jokes_for_creator",
      payload: sample.creator
    });
    assert.equal(linksToCreators.length, 0);

    // Bob gets the deleted Creators for the Joke 
    const deletedLinksToCreators: Array<[SignedActionHashed<CreateLink>, SignedActionHashed<DeleteLink>[]]> = await bob.cells[0].callZome({
      zome_name: "jokes",
      fn_name: "get_deleted_jokes_for_creator",
      payload: sample.creator
    });
    assert.equal(deletedLinksToCreators.length, 1);

  });
});
