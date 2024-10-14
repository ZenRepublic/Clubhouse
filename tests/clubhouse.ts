import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Clubhouse } from "../target/types/clubhouse";

describe("clubhouse", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Clubhouse as Program<Clubhouse>;

  it("Is initialized!", async () => {
    const program = anchor.workspace.Clubhouse as Program<Clubhouse>;
  });
});
