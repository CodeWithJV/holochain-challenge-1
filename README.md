# Challenge 1 - Actions and Entries

In this challenge we are going to create a joke app, where each user (agent) can create jokes and store them on the DHT.

A large portion of the code has been scaffolded using `hc scaffold` however, we've taken bits out to help you understand what the code generated by hc scaffold is actually doing.

There will be multiple operations that each agent can do on there jokes:

- [ ] 1. An agent can create a joke
- [ ] 2. An agent can get a joke, by that jokes Action Hash
- [ ] 3. And an agent can edit jokes
- [ ] 4. An agent can delete jokes

## Create a joke

#### 1. Fork this repo and clone it down

#### 2. run `nix develop`.

#### 3. run `npm i` to install packages

#### 4. run `npm start` and open up the holochain playground

#### 5. On one the agents windows, enter some text into the text field and click the create joke button.

You will notice the error popup: `Attempted to call a zome function that doesn't exist: Zome: jokes Fn create_joke`

<details>
<summary>
Tip!
</summary>

If you see the error
`sh: 1: hc: Permission denied` in your terminal, it means you forgot to run `nix develop`!

</details>

#### 5. Open up the browser console, navigate to where the error occured. You will see our frontend is trying to make a call to a backend zome function. We need to implement this function on the holochain app to solve the error.

<details>
<summary>
Hint
</summary>

Press `F12` or `right click > inspect element` to open up the dev tools. Select `console` you should then see the error.

The error inside the console should point us to our [CreateJoke.svelte](ui/src/jokes/jokes/CreateJoke.svelte) file

</details>

#### 6. navigate to `dnas/jokes/zomes/coordinator/jokes/src/joke.rs` and paste the following code at the top of the file, underneath the `use` statements

```rust
#[hdk_extern]
pub fn create_joke(joke: Joke) -> ExternResult<Record> {
    let joke_hash = create_entry(&EntryTypes::Joke(joke.clone()))?;
    let record = get(joke_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly created Joke")))
    )?;
    Ok(record)
}

```

#### 7. Save the file, restart the holochain application and create another joke.

If you look back at the playground, this time you should see a new entry has been created in the DHT.

<details>
<summary>
Hint
</summary>

Press `Ctrl + C` twice to stop the process, and `npm start` to start it again

</details>

#### 8. Select each DHT cell inside the dht-cell panel.

You will notice that the source chain of each cell is different. The cell of the Agent who created the joke, contains an **entry** and its corresponding **create action**, and the other cell will not have this.

## Get a Joke from another agent

To retrieve an agents joke from the DHT, we are going to need the Action Hash of that joke.

Inside `c-1/ui/src/App.svelte`, we are going to create a text field for where we can input an action hash.

#### 1. Start by declaring a variable that holds the state for the text field inside the `<script>` block

```ts
let jokeHash = ''
$: jokeHash
```

#### 2. Next we can create our text field element. Paste this code just below where the `CreateJoke` component is implemented, inside the `<main>` block

```svelte
<h3 style="margin-bottom: 16px; margin-top: 32px;">Retrieve A Joke!</h3>
<mwc-textfield
    type="text"
    placeholder="Enter the action hash of a joke..."
    value={jokeHash}
    on:input={(e) => {
    jokeHash = e.currentTarget.value
    }}
    required
    style="margin-bottom: 16px;"
/>
```

#### 3. Save `App.svelte`, and head back into one of the agent windows. You should see the text field displayed

Notice how we didn't need to completely restart the app this time? Svelte uses a technique called Hot Reloading to update changes made to the UI almost instantly!

#### 4. We are going to need to add a button which triggers the retrieval of a joke from the DHT, and then displays it for the user. To do this, we will use the UI component `JokeDetail`, as well as another piece of state to manage its visibility.

Place these lines of code inside the same `script` tag of `App.svelte`

```ts
import JokeDetail from './jokes/jokes/JokeDetail.svelte'
```

```ts
let showJoke = false
$: showJoke
```

#### 5. Next we can add our button and UI component to render the retrieved joke. Place this block of code underneath the text-input we added before

```svelte
<!-- svelte-ignore a11y-click-events-have-key-events -->
<mwc-button
    raised
    on:click={() => {
    showJoke = true
    }}
>
    Get Joke
</mwc-button>
{#if showJoke}
    <JokeDetail jokeHash={decodeHashFromBase64(jokeHash)} />
{/if}
```

#### 7. Navigate back to `dnas/jokes/zomes/coordinator/jokes/src/joke.rs` and paste the following code underneath our `create_joke` function

```rust
#[hdk_extern]
pub fn get_joke_by_hash(original_joke_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_joke_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => {
            Err(wasm_error!(WasmErrorInner::Guest(String::from("Malformed get details response"))))
        }
    }
}

```

This zome function is called by the `JokeDetail` component when it mounts. It takes in the action hash for the joke as an argument, and then returns the record corresponding to it.

#### 8. Save the file, restart the holochain app and create a new joke inside an Agent's window.

#### 9. Inside that same agents window, open up the console, copy the hash of the action just created, paste it into the other Agents Get Joke text field, and press the **Get Joke** button.

You should see your newely created joke render on the UI!

## Edit a joke

You may have noticed that when we retrieve a joke, our `JokeDetail` component displays the joke text, as well as an option to edit and delete the joke.

#### 1. Create a joke, retrieve it, and then click on the edit button next it

#### 2. Change the joke to another bit of text, and press **Save**

You will notice nothing will happen. Once again, we will need to implement some code to get this working.

#### 3. Navigate to `c-1/ui/src/jokes/jokes/EditJoke.svelte`

This `EditJoke` component holds the code for the UI where we can edit jokes. It is already implemented inside the `JokeDetail` component.

#### 4. Find the `updateJoke` function and paste the following code inside of it.

```ts
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
```

When the `save` button is clicked in the UI this block of code will make a call to the backend Zome function `update_joke`.

#### 5. Save the file, navigate back to `dnas/jokes/zomes/coordinator/jokes/src/joke.rs` and paste the following code underneath our `get_joke_by_hash` function

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateJokeInput {
    pub original_joke_hash: ActionHash,
    pub previous_joke_hash: ActionHash,
    pub updated_joke: Joke,
}

#[hdk_extern]
pub fn update_joke(input: UpdateJokeInput) -> ExternResult<Record> {
    let updated_joke_hash = update_entry(input.previous_joke_hash.clone(), &input.updated_joke)?;
    let record = get(updated_joke_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly updated Joke")))
    )?;
    Ok(record)
}
```

Notice how this block of code contains a struct as well as the Zome function. For this update function, we want to send multiple bits of data from the client, but Zome functions can only a take a single parameter. Using a struct type allows us to circumvent this.

#### 6. Save the file, and restart the app.

#### 7. Create a joke, retrieve it, edit its contents, and press `save`.

#### 8. Look at the source chain for the cell we just edited a joke for. You will see another action has been added.

It's important to understand how updates in Holochain work. When you commence an update action, it will not update the contents of the entry. Instead it will add another entry to to the DHT, and mark the previous original entry as outdated.

This applies to delete actions as well, and it means that any entries once added to the DHT will remain on it forever.

#### 9. Refresh the agents windows using `F5` or `Ctrl + R`

#### 10. In each agent window, try retrieve both the original create action and the new update action

You will notice the updated entry contains the latest text you inputted, and the data inside the entry of the original create action is still accessible.

## Delete a joke

#### 1. Navigate to the `deleteJoke` function inside `JokeDetail.svelte`, and write code to create a zome call to `delete_joke`.

- The payload should be the `jokeHash`

#### 2. Save the file, navigate to `dnas/jokes/zomes/coordinator/jokes/src/joke.rs` and write a zome function to delete a joke

Try figure it out yourself!

<details>
<summary>
Hint!
</summary>

```rust
#[hdk_extern]
pub fn delete_joke(original_joke_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_joke_hash)
}

```

</details>

#### 3. Save the file and restart the app

#### 4. Create a joke, retrieve it, and then delete it using the delete button.

Just like with editing and creating a joke, deleting a joke adds another action the the Agents source chain.

On the agent window, there won't be any visual indicator that you've deleted the entry, and you will still be able to retrieve the original create action.

Just like with update actions, delete actions don't actually delete entries/actions from the DHT. They just mark them as outdated. However, when retrieving an old action for which the entry previously existed, it remains accessible.

You may be thinking, "What's the point of a delete action if I can still access the old entry?" - More on this in a future challenge!

Well done! You made it to the end.
