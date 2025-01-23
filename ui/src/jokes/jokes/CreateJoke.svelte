<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import {encodeHashToBase64} from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Joke } from "./types";

const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let text: string = "";

export let creator!: AgentPubKey;

$: text, creator;
$: isJokeValid = true && text !== "";

onMount(async () => {
  if (creator === undefined) {
    throw new Error(`The creator input is required for the CreateJoke element`);
  }
  client = await appClientContext.getClient();
});

async function createJoke() {
  const jokeEntry: Joke = {
    text: text!,
    creator: creator!,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "jokes",
      zome_name: "jokes",
      fn_name: "create_joke",
      payload: jokeEntry,
    });
    console.log(encodeHashToBase64(record.signed_action.hashed.hash))
    dispatch("joke-created", { jokeHash: record.signed_action.hashed.hash });
  } catch (e) {
    console.error(e)
    alert((e as HolochainError).message);
  }
}
</script>

<div>
  <h3>Create Joke</h3>

  <div>
    <label for="Text">Text</label>
    <textarea name="Text" bind:value={text} required />
  </div>

  <button disabled={!isJokeValid} on:click={() => createJoke()}>
    Create Joke
  </button>
</div>
