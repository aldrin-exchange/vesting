import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import { errLogs, provider, payer, getErr } from "../helpers";
import { Vesting } from "../vesting";

export function test() {
  describe("fund_vesting_vault", () => {
    const adminKeypair = Keypair.generate();
    const walletAuthority = Keypair.generate();
    let fundingWallet: PublicKey;
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

    beforeEach("create vesting account", async () => {
      vesting = await Vesting.init({
        adminKeypair,
        // vesteeWallet,
        mint: vestingMint,
      });
    });

    beforeEach("create funding wallet", async () => {
        fundingWallet = await createAccount(
          provider.connection,
          payer,
          vestingMint,
          walletAuthority.publicKey
        );

        await mintTo(
            provider.connection,
            payer,
            vestingMint,
            fundingWallet,
            payer.publicKey,
            1_000_000
        );
      });

    it("works", async () => {
      const vestingInfoBefore = await vesting.fetch();

      await vesting.fundVestingVault({walletAuthority,fundingWallet}, 5_000);
      const vestingInfoAfter1 = await vesting.fetch();
      await vesting.fundVestingVault({walletAuthority,fundingWallet}, 2_000);
      const vestingInfoAfter2 = await vesting.fetch();

      // Check that the vestingVaultBalance is correct
      expect(vestingInfoBefore.vestingVaultBalance.amount.toNumber()).to.eq(0);
      expect(vestingInfoAfter1.vestingVaultBalance.amount.toNumber()).to.eq(5_000);
      expect(vestingInfoAfter2.vestingVaultBalance.amount.toNumber()).to.eq(7_000);

      // Everything else remains with the same default values
      expect(vestingInfoAfter2.totalVestingAmount.amount.toNumber()).to.eq(
        10_000
      );
      expect(vestingInfoAfter2.startTs.time.toNumber()).to.eq(1577836801);
      expect(vestingInfoAfter2.cliffPeriods.toNumber()).to.eq(12);
      expect(vestingInfoAfter2.totalPeriods.toNumber()).to.eq(48);
      expect(vestingInfoAfter2.periodType).to.deep.eq({ monthly: {} });
      expect(vestingInfoAfter2.admin).to.deep.eq(adminKeypair.publicKey);
      expect(vestingInfoAfter2.mint).to.deep.eq(vestingMint);
      expect(vestingInfoAfter2.vault).to.deep.eq(await vesting.vestingVault());
      expect(vestingInfoAfter2.cumulativeWithdrawnAmount.amount.toNumber()).to.eq(0);
      expect(vestingInfoAfter2.unfundedLiabilities.amount.toNumber()).to.eq(0);
    });
  });
}
