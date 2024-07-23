<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte'
  import '@material/mwc-circular-progress'
  import { decode } from '@msgpack/msgpack'
  import {
    type AppClient,
    type Record,
    type ActionHash,
    encodeHashToBase64,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Joke } from './types'
  import '@material/mwc-circular-progress'
  import type { Snackbar } from '@material/mwc-snackbar'
  import '@material/mwc-snackbar'
  import '@material/mwc-icon-button'
  import EditJoke from './EditJoke.svelte'

  const dispatch = createEventDispatcher()

  export let jokeHash: ActionHash

  let client: AppClient = (getContext(clientContext) as any).getClient()

  let loading: boolean
  let error: any = undefined

  let record: Record | undefined
  let joke: Joke | undefined

  let editing = false

  let errorSnackbar: Snackbar

  $: editing, error, loading, record, joke

  $: if (jokeHash) {
    fetchJoke();
  }

  onMount(async () => {
    if (jokeHash === undefined) {
      throw new Error(
        `The jokeHash input is required for the JokeDetail element`
      )
    }
  })

  async function fetchJoke() {
    loading = true

    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'jokes',
        zome_name: 'jokes',
        fn_name: 'get_joke_by_hash',
        payload: jokeHash,
      })
      if (record) {
        joke = decode((record.entry as any).Present.entry) as Joke
      }
    } catch (e) {
      error = e
    }

    loading = false
  }

  async function deleteJoke() {
    // Implement Joke delete logic here
    // ...
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>

{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{:else if error}
  <br />
  <span>Error fetching the joke: {error}</span>
{:else if editing}
  <EditJoke
    originalJokeHash={jokeHash}
    currentRecord={record}
    on:joke-updated={async () => {
      editing = false
      await fetchJoke()
    }}
    on:edit-canceled={() => {
      editing = false
    }}
  ></EditJoke>
{:else}
  <div style="display: flex; flex-direction: column">
    <div style="display: flex; flex-direction: row">
      <span style="flex: 1"></span>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <mwc-icon-button
        style="margin-left: 8px"
        icon="edit"
        on:click={() => {
          editing = true
        }}
      ></mwc-icon-button>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <mwc-icon-button
        style="margin-left: 8px"
        icon="delete"
        on:click={() => deleteJoke()}
      ></mwc-icon-button>
    </div>

    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span style="margin-right: 4px"><strong>Text:</strong></span>
      <span style="white-space: pre-line">{joke?.text}</span>
    </div>
  </div>
{/if}
