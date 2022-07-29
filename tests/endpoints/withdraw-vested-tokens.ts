import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import {
  createMint,
  createAccount,
  mintTo,
} from "@solana/spl-token";
import { errLogs, provider, payer } from "../helpers";
import { Vesting } from "../vesting";

export function test() {
  describe("withdraw_vested_tokens", () => {
    const adminKeypair = Keypair.generate();
    const walletAuthority = Keypair.generate();
    let vesteeWallet: PublicKey;
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

    beforeEach("create vestee wallet", async () => {
      vesteeWallet = await createAccount(
        provider.connection,
        payer,
        vestingMint,
        payer.publicKey,
        Keypair.generate()
      );
    });

    beforeEach("create vesting account", async () => {
      vesting = await Vesting.init({
        adminKeypair,
        vesteeWallet,
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

    it("fails if pda is not correct", async () => {
      const logs = await errLogs(
        vesting.withdrawVestedTokens(
          { pda: Keypair.generate().publicKey, vesteeWallet },
          10
        )
      );
      expect(logs).to.contain("seeds constraint was violated");
    });

    it("fails if wrong vestee wallet", async () => {
      const fakeMint = await createMint(
        provider.connection,
        payer,
        payer.publicKey,
        null,
        9
      );
      const fakeWallet = await createAccount(
        provider.connection,
        payer,
        fakeMint,
        walletAuthority.publicKey
      );

      const logs = await errLogs(
        vesting.withdrawVestedTokens({ vesteeWallet: fakeWallet }, 10)
      );

      expect(logs).to.contain(
        "Vestee wallet input does not match the vestee wallet in the vesting account"
      );
    });

    it("fails if wrong vesting vault", async () => {
      const fakeVault = await createAccount(
        provider.connection,
        payer,
        vestingMint,
        payer.publicKey,
        Keypair.generate()
      );

      const logs = await errLogs(
        vesting.withdrawVestedTokens(
          { vestingVault: fakeVault, vesteeWallet },
          10
        )
      );

      expect(logs).to.contain(
        "Vault input does not match the vault in the vesting account"
      );
    });

    it("works", async () => {
      await vesting.updateVestedTokens();
      const vestingInfoBefore = await vesting.fetch();
      const vestedAmount =
        vestingInfoBefore.cumulativeVestedAmount.amount.toNumber();

      expect(vestingInfoBefore.unfundedLiability.amount.toNumber()).to.eq(
        vestedAmount
      );

      await vesting.fundVestingVault(
        { walletAuthority, fundingWallet },
        vestedAmount - 10
      );
      await vesting.withdrawVestedTokens({ vesteeWallet }, vestedAmount - 15);

      const vestingInfoAfter = await vesting.fetch();

      expect(vestingInfoAfter.cumulativeVestedAmount.amount.toNumber()).to.eq(
        vestedAmount
      );
      expect(vestingInfoAfter.unfundedLiability.amount.toNumber()).to.eq(10);
      expect(vestingInfoAfter.vaultBalance.amount.toNumber()).to.eq(5);
      expect(
        vestingInfoAfter.cumulativeWithdrawnAmount.amount.toNumber()
      ).to.eq(vestedAmount - 15);

      // Everything else remains with the same default values
      expect(vestingInfoAfter.totalVestingAmount.amount.toNumber()).to.eq(
        10_000
      );
      expect(vestingInfoAfter.startTs.time.toNumber()).to.eq(1577836801);
      expect(vestingInfoAfter.cliffPeriods.toNumber()).to.eq(12);
      expect(vestingInfoAfter.totalPeriods.toNumber()).to.eq(48);
      expect(vestingInfoAfter.periodType).to.deep.eq({ monthly: {} });
      expect(vestingInfoAfter.admin).to.deep.eq(adminKeypair.publicKey);
      expect(vestingInfoAfter.mint).to.deep.eq(vestingMint);
      expect(vestingInfoAfter.vault).to.deep.eq(await vesting.vestingVault());
    });
  });
}
