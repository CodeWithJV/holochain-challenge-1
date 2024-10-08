<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from 'svelte'
  import {
    type AppClient,
    type Record,
    type AgentPubKey,
    type ActionHash,
    encodeHashToBase64,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Joke } from './types'
  import '@material/mwc-button'
  import '@material/mwc-snackbar'
  import type { Snackbar } from '@material/mwc-snackbar'

  import '@material/mwc-textarea'
  let client: AppClient = (getContext(clientContext) as any).getClient()

  const dispatch = createEventDispatcher()

  export let creator!: AgentPubKey

  let text: string = ''

  let errorSnackbar: Snackbar

  $: text, creator
  $: isJokeValid = true && text !== ''

  onMount(() => {
    if (creator === undefined) {
      throw new Error(
        `The creator input is required for the CreateJoke element`
      )
    }
  })

  async function createJoke() {
    const jokeEntry: Joke = {
      text: text!,
      creator: creator!,
    }

    try {
      // Create the Joke
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'jokes',
        zome_name: 'jokes',
        fn_name: 'create_joke',
        payload: jokeEntry,
      })

      console.log(
        `ACTION HASH: ${encodeHashToBase64(record.signed_action.hashed.hash)}`
      )

      dispatch('joke-created', { jokeHash: record.signed_action.hashed.hash })
      text = ''
    } catch (e) {
      console.error(`Error creating the joke: ${e}`)
      errorSnackbar.labelText = `Error creating the joke: ${e}`
      errorSnackbar.show()
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <h3>Create A Joke!</h3>
  <br />
  <div style="margin-bottom: 16px">
    <mwc-textarea
      outlined
      label="Text"
      value={text}
      on:input={(e) => {
        text = e.target.value
      }}
      required
      style="width: 100%"
    ></mwc-textarea>
  </div>

  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <mwc-button
    outlined
    label="Create Joke"
    disabled={!isJokeValid}
    on:click={() => createJoke()}
  ></mwc-button>
</div>
