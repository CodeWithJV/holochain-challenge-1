<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from 'svelte'
  import type { AppClient, Record, ActionHash } from '@holochain/client'
  import { encodeHashToBase64 } from '@holochain/client'
  import { decode } from '@msgpack/msgpack'
  import { clientContext } from '../../contexts'
  import type { Joke } from './types'
  import '@material/mwc-button'
  import '@material/mwc-snackbar'
  import type { Snackbar } from '@material/mwc-snackbar'

  import '@material/mwc-textarea'
  let client: AppClient = (getContext(clientContext) as any).getClient()

  const dispatch = createEventDispatcher()

  export let originalJokeHash!: ActionHash

  export let currentRecord!: Record
  let currentJoke: Joke = decode(
    (currentRecord.entry as any).Present.entry
  ) as Joke

  let text: string | undefined = currentJoke.text

  let errorSnackbar: Snackbar

  $: text
  $: isJokeValid = true && text !== ''

  onMount(() => {
    if (currentRecord === undefined) {
      throw new Error(
        `The currentRecord input is required for the EditJoke element`
      )
    }
    if (originalJokeHash === undefined) {
      throw new Error(
        `The originalJokeHash input is required for the EditJoke element`
      )
    }
  })

  async function updateJoke() {
    // Implement Joke update/edit logic here
    // ...
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Joke</span>

  <div style="margin-bottom: 16px">
    <mwc-textarea
      outlined
      label="Text"
      value={text}
      on:input={(e) => {
        text = e.target.value
      }}
      required
    ></mwc-textarea>
  </div>

  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button
      raised
      label="Save"
      disabled={!isJokeValid}
      on:click={() => updateJoke()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
