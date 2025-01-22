<script lang="ts">
import type { ActionHash, AppClient, HolochainError } from "@holochain/client";
import { AppWebsocket, decodeHashFromBase64  } from "@holochain/client";
import { onMount, setContext } from "svelte";

import logo from "./assets/holochainLogo.svg";
import { type ClientContext, clientContext } from "./contexts";

import CreateJoke from './jokes/jokes/CreateJoke.svelte'
import Banner from './Banner.svelte'

// Import the JokeDetail component here
// ...
import JokeDetail from './jokes/jokes/JokeDetail.svelte'

let client: AppClient | undefined;
let error: HolochainError | undefined;
let loading = false;

// Paste your variable and state declarations here
// ...
let jokeHash = ''
let retrieveJokeHash = ''
$: jokeHash

const appClientContext = {
  getClient: async () => {
    if (!client) {
      client = await AppWebsocket.connect();
    }
    return client;
  },
};

onMount(async () => {
  try {
    loading = true;
    client = await appClientContext.getClient();
  } catch (e) {
  console.log(e)
    error = e as HolochainError;
  } finally {
    loading = false;
  }
});

setContext<ClientContext>(clientContext, appClientContext);
</script>

<Banner challengeName={'Entries and Actions'} challengeNumber={1}>
  <div
    style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; margin-left: auto; margin-right: auto; max-width: 600px;"
  >
    {#if loading || !client}
      <div
        style="display: flex; flex: 1; align-items: center; justify-content: center"
      >
        Loading
      </div>
    {:else}
      <div
        id="content"
        style="display: flex; flex-direction: column; flex: 1; margin-bottom: 15%;"
      >
        <!-- Place your other code here -->
        <CreateJoke creator={client?.myPubKey} />
<h3 style="margin-bottom: 16px; margin-top: 32px;">Retrieve A Joke!</h3>
<input
    type="text"
    placeholder="Enter the action hash of a joke..."
    value={jokeHash}
    on:input={(e) => {
    jokeHash = e.currentTarget.value
    }}
    required
    style="margin-bottom: 16px;"
/>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<button
    on:click={() => {
      retrieveJokeHash = undefined //force reload of joke detail component
      retrieveJokeHash = jokeHash
    }}
>
    Get Joke
</button>
{#if retrieveJokeHash}
    <JokeDetail jokeHash={decodeHashFromBase64(retrieveJokeHash)} />
{/if}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
      </div>
    {/if}
  </div>
</Banner>

<style>
</style>
