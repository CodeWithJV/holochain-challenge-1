import {
  ActionHash,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeDnaHash,
  fakeEntryHash,
  hashFrom32AndType,
  NewEntryAction,
  Record,
} from "@holochain/client";
import { CallableCell } from "@holochain/tryorama";

export async function sampleJoke(cell: CallableCell, partialJoke = {}) {
  return {
    ...{
      text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      creator: cell.cell_id[1],
    },
    ...partialJoke,
  };
}

export async function createJoke(cell: CallableCell, joke = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "jokes",
    fn_name: "create_joke",
    payload: joke || await sampleJoke(cell),
  });
}
