import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Clubhouse } from "../target/types/clubhouse";

describe("clubhouse", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Clubhouse as Program<Clubhouse>;

  it("Is initialized!", async () => {
    const program = anchor.workspace.Clubhouse as Program<Clubhouse>;
    var res = await program.methods.addProgramAdmin().accounts({
      signer: anchor.workspace.Clubhouse.provider.wallet.publicKey,
      programAdmin: new anchor.web3.PublicKey("RDWdaPcyAwpbBZyK1vZeFAFYsn5jjnRJZNRqdeJD9Y7"),
    }).rpc();
  });
});
