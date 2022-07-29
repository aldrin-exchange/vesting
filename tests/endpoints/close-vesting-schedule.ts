import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { createMint, createAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { errLogs, provider, payer, getErr } from "../helpers";
import { Vesting } from "../vesting";

export function test() {
  describe("close_vesting_schedule", () => {
    let vesteeWallet: PublicKey;
    let vestingMint: PublicKey;

    beforeEach("create vesting mint", async () => {
      vestingMint = await createMint(
        provider.connection,
        payer,
        payer.publicKey,
        null,
        9
      );
    });

    beforeEach("create vestee wallet", async () => {
      vesteeWallet = await createAccount(
        provider.connection,
        payer,
        vestingMint,
        payer.publicKey
      );
    });

    it("fails if wrong admin", async () => {
    
    });

    it("fails if not fully vested", async () => {
      
    });

    it("fails if not fully withdrawn", async () => {
      
    });

    it("works", async () => {
    });
  });
}
