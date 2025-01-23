<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import {encodeHashToBase64} from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Joke } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

export let currentRecord!: Record;
export let originalJokeHash!: ActionHash;

let currentJoke: Joke = decode((currentRecord.entry as any).Present.entry) as Joke;
let text: string | undefined = currentJoke.text;

$: text;
$: isJokeValid = true && text !== "";

onMount(async () => {
  if (!currentRecord) {
    throw new Error(`The currentRecord input is required for the EditJoke element`);
  }
  if (!originalJokeHash) {
    throw new Error(`The originalJokeHash input is required for the EditJoke element`);
  }
  client = await appClientContext.getClient();
});

async function updateJoke() {
  // Implement Joke update/edit logic here
  const joke: Joke = {
    text: text!,
    creator: currentJoke.creator,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: "jokes",
      zome_name: "jokes",
      fn_name: "update_joke",
      payload: {
        original_joke_hash: originalJokeHash,
        previous_joke_hash: currentRecord.signed_action.hashed.hash,
        updated_joke: joke,
      },
    });

    console.log(
      `NEW ACTION HASH: ${encodeHashToBase64(updateRecord.signed_action.hashed.hash)}`
    )

    dispatch("joke-updated", { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<section>
  <div>
    <label for="Text">Text</label>
    <textarea name="Text" bind:value={text} required />
  </div>

  <div>
    <button on:click={() => dispatch("edit-canceled")}>Cancel</button>
    <button disabled={!isJokeValid} on:click={() => updateJoke()}>
      Edit Joke
    </button>
  </div>
</section>
