<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from 'svelte'
  import {
    type AppClient,
    type Record,
    type EntryHash,
    type AgentPubKey,
    type DnaHash,
    type ActionHash,
    encodeHashToBase64,
  } from '@holochain/client'
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
    const joke: Joke = {
      text: text!,
      creator: currentJoke.creator,
    }

    try {
      const updateRecord: Record = await client.callZome({
        cap_secret: null,
        role_name: 'jokes',
        zome_name: 'jokes',
        fn_name: 'update_joke',
        payload: {
          original_joke_hash: originalJokeHash,
          previous_joke_hash: currentRecord.signed_action.hashed.hash,
          updated_joke: joke,
        },
      })
      console.log(
        `HASH: ${encodeHashToBase64(updateRecord.signed_action.hashed.hash)}`
      )

      dispatch('joke-updated', {
        actionHash: updateRecord.signed_action.hashed.hash,
      })
    } catch (e) {
      errorSnackbar.labelText = `Error updating the joke: ${e}`
      errorSnackbar.show()
    }
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
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <mwc-button
      raised
      label="Save"
      disabled={!isJokeValid}
      on:click={() => updateJoke()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
