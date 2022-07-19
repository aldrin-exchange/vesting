import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VestingTreasury } from "../target/types/vesting_treasury";

describe("vesting-treasury", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VestingTreasury as Program<VestingTreasury>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
