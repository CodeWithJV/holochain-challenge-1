<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import EditJoke from "./EditJoke.svelte";
import type { Joke } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

let loading: boolean = false;
let editing = false;
let error: HolochainError | undefined;
let record: Record | undefined;
let joke: Joke | undefined;

export let jokeHash: ActionHash;

$: editing, error, loading, record, joke;

onMount(async () => {
  if (jokeHash === undefined) {
    throw new Error(`The jokeHash input is required for the JokeDetail element`);
  }
  client = await appClientContext.getClient();
  await fetchJoke();
});

async function fetchJoke() {
  loading = true;
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: "jokes",
      zome_name: "jokes",
      fn_name: "get_joke_by_hash",
      payload: jokeHash,
    });
    if (record) {
      joke = decode((record.entry as any).Present.entry) as Joke;
    }
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}

async function deleteJoke() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: "jokes",
      zome_name: "jokes",
      fn_name: "delete_joke",
      payload: jokeHash,
    });
    dispatch("joke-deleted", { jokeHash: jokeHash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching the joke: {error.message}</div>
{:else if editing}
  <EditJoke
    originalJokeHash={jokeHash}
    currentRecord={record}
    on:joke-updated={async () => {
      editing = false;
      await fetchJoke();
    }}
    on:edit-canceled={() => {
      editing = false;
    }}
  />
{:else}
  <section>
    <div>
      <span><strong>Text:</strong></span>
      <span>{joke?.text}</span>
    </div>

    <div>
      <button
        on:click={() => {
          editing = true;
        }}
      >edit</button>
      <button on:click={() => deleteJoke()}>delete</button>
    </div>
  </section>
{/if}
