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

let client: AppClient | undefined;
let error: HolochainError | undefined;
let loading = false;

// Paste your variable and state declarations here
// ...

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
        <CreateJoke creator={client?.myPubKey} />
        <!-- Place your other code here -->
        <!-- svelte-ignore a11y-click-events-have-key-events -->
      </div>
    {/if}
  </div>
</Banner>

<style>
</style>
