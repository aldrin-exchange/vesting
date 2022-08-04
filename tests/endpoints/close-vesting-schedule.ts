import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { createMint, createAccount, mintTo } from "@solana/spl-token";
import { errLogs, provider, payer, getErr } from "../helpers";
import { Vesting } from "../vesting";

export function test() {
  describe("close_vesting_schedule", () => {
    const adminKeypair = Keypair.generate();
    const walletAuthority = Keypair.generate();
    let vesteeWallet: PublicKey;
    let vestingMint: PublicKey;
    let fundingWallet: PublicKey;
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
        // needs to be sufficienly in the past
        // such that the contract is fully vested
        startTs: 1262304001,
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

    it("fails if wrong admin", async () => {
      const fakeAdminKeypair = Keypair.generate();

      const logs = await errLogs(
        vesting.closeVestingSchedule({ adminKeypair: fakeAdminKeypair })
      );

      expect(logs).to.contain("Admin does not own this vesting account");
    });

    it("fails if not fully vested", async () => {
      const vesting2 = await Vesting.init({
        adminKeypair,
        vesteeWallet,
        mint: vestingMint,
      });

      await vesting2.updateVestedTokens();

      const logs = await errLogs(
        vesting2.closeVestingSchedule({ adminKeypair })
      );

      expect(logs).to.contain("This vesting account is not fully vested");
    });

    it("fails if not fully withdrawn", async () => {
      await vesting.updateVestedTokens();
      const vestingInfo = await vesting.fetch();
      const vestedAmount = vestingInfo.cumulativeVested.amount.toNumber();

      await vesting.fundVestingVault(
        { walletAuthority, fundingWallet },
        vestedAmount
      );
      await vesting.withdrawVestedTokens({ vesteeWallet }, vestedAmount - 10);

      const logs = await errLogs(
        vesting.closeVestingSchedule({ adminKeypair })
      );

      expect(logs).to.contain(
        "This vested tokens of this vesting account are not fully withdrawn"
      );
    });

    it("works", async () => {
      await vesting.updateVestedTokens();
      const vestingInfo = await vesting.fetch();
      const vestedAmount = vestingInfo.cumulativeVested.amount.toNumber();

      await vesting.fundVestingVault(
        { walletAuthority, fundingWallet },
        vestedAmount
      );
      await vesting.withdrawVestedTokens({ vesteeWallet }, vestedAmount);

      await vesting.closeVestingSchedule({ adminKeypair });

      const logs = await getErr(vesting.fetch());

      expect(logs).to.contain("Account does not exist");
    });
  });
}
