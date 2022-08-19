import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { createMint, createAccount } from "@solana/spl-token";
import { errLogs, provider, payer } from "../helpers";
import { Vesting } from "../vesting";

export function test() {
  describe("update_vested_tokens", () => {
    const adminKeypair = Keypair.generate();
    let vesteeWallet: PublicKey;
    let vestingMint: PublicKey;
    let vesting: Vesting;

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

    beforeEach("create vesting account", async () => {
      vesting = await Vesting.init({
        adminKeypair,
        vesteeWallet,
        mint: vestingMint,
      });
    });

    it("works", async () => {
      const vestingInfoBefore = await vesting.fetch();

      await vesting.updateVestedTokens();

      const vestingInfoAfter = await vesting.fetch();

      // Check that the cumulativeVestedAmount and unfunded liability
      // changes state
      expect(vestingInfoBefore.cumulativeVested.amount.toNumber()).to.eq(0);
      expect(vestingInfoAfter.cumulativeVested.amount.toNumber()).to.above(0);
      expect(vestingInfoBefore.unfundedLiability.amount.toNumber()).to.eq(0);
      expect(vestingInfoAfter.unfundedLiability.amount.toNumber()).to.above(0);

      // Everything else remains with the same default values
      expect(vestingInfoAfter.totalVesting.amount.toNumber()).to.eq(10_000);
      expect(vestingInfoAfter.startTs.time.toNumber()).to.eq(1577836801);
      expect(vestingInfoAfter.cliffPeriods.toNumber()).to.eq(12);
      expect(vestingInfoAfter.totalPeriods.toNumber()).to.eq(48);
      expect(vestingInfoAfter.periodType).to.deep.eq({ monthly: {} });
      expect(vestingInfoAfter.admin).to.deep.eq(adminKeypair.publicKey);
      expect(vestingInfoAfter.mint).to.deep.eq(vestingMint);
      expect(vestingInfoAfter.vault).to.deep.eq(await vesting.vestingVault());
      expect(vestingInfoAfter.cumulativeWithdrawn.amount.toNumber()).to.eq(0);
      expect(vestingInfoAfter.vaultBalance.amount.toNumber()).to.eq(0);
    });
  });
}
