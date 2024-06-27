<script lang="ts">
  import { onMount, setContext } from 'svelte'
  import type { ActionHash, AppClient } from '@holochain/client'
  import { AppWebsocket, decodeHashFromBase64 } from '@holochain/client'
  import '@material/mwc-circular-progress'

  import { clientContext } from './contexts'

  import CreateJoke from './jokes/jokes/CreateJoke.svelte'

  // Import the JokeDetail component here
  // ...
  import JokeDetail from './jokes/jokes/JokeDetail.svelte'

  let client: AppClient | undefined

  let loading = true

  // Paste your variable and state declarations here
  // ...
  let jokeHash = ''
  $: jokeHash
  let showJoke = false
  $: showJoke

  onMount(async () => {
    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    client = await AppWebsocket.connect()

    loading = false
  })

  setContext(clientContext, {
    getClient: () => client,
  })
</script>

<main>
  {#if loading}
    <div
      style="display: flex; flex: 1; align-items: center; justify-content: center"
    >
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <div id="content" style="display: flex; flex-direction: column; flex: 1;">
      <CreateJoke creator={client?.myPubKey} />
      <!-- Place your other code here -->
      <input
        type="text"
        placeholder="Enter the action hash of a joke..."
        value={jokeHash}
        on:input={(e) => {
          jokeHash = e.currentTarget.value
        }}
        required
      />
      <button
        on:click={() => {
          showJoke = true
        }}
      >
        Get Joke
      </button>
      {#if showJoke}
        <JokeDetail jokeHash={decodeHashFromBase64(jokeHash)} />
      {/if}
    </div>
  {/if}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
