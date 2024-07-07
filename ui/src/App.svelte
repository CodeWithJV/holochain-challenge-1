<script lang="ts">
  import { onMount, setContext } from 'svelte'
  import type { ActionHash, AppClient } from '@holochain/client'
  import { AppWebsocket, decodeHashFromBase64 } from '@holochain/client'
  import '@material/mwc-circular-progress'
  import '@material/mwc-textfield'

  import { clientContext } from './contexts'

  import CreateJoke from './jokes/jokes/CreateJoke.svelte'
  import Banner from './Banner.svelte'

  // Import the JokeDetail component here
  // ...

  let client: AppClient | undefined

  let loading = true

  // Paste your variable and state declarations here
  // ...

  onMount(async () => {
    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    client = await AppWebsocket.connect()

    loading = false
  })

  setContext(clientContext, {
    getClient: () => client,
  })
</script>

<Banner challengeName={'Entries and Actions'} challengeNumber={1}>
  <div
    style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; margin-left: auto; margin-right: auto; max-width: 600px;"
  >
    {#if loading}
      <div
        style="display: flex; flex: 1; align-items: center; justify-content: center"
      >
        <mwc-circular-progress indeterminate />
      </div>
    {:else}
      <div
        id="content"
        style="display: flex; flex-direction: column; flex: 1; margin-bottom: 15%;"
      >
        <br />
        <br />
        <br />
        <!-- Place your other code here -->
        <CreateJoke creator={client?.myPubKey} />
        <br />
        <!-- svelte-ignore a11y-click-events-have-key-events -->
      </div>
    {/if}
  </div>
</Banner>
