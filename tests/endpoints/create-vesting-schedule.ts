import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { VestingTreasury } from "../../target/types/vesting_treasury";
import { createMint, createAccount, getAccount, mintTo, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { errLogs, provider, payer } from "../helpers";
import { Vesting } from "../vesting";
import { PublicKeyword } from "typescript";

export function test() {
  describe("create_vesting_schedules", () => {
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

    // it("fails if wallet account isn't initialized", async () => {
    //   const fakeWallet = await createAccount(
    //     provider.connection,
    //     payer,
    //     vestingMint,
    //     payer.publicKey
    //   );

    //   const logs = await errLogs(Vesting.init(
    //     {
    //       vesteeWallet: fakeWallet
    //     },
    //     10_000,
    //     1577836801, // start
    //     1609459201, // cliff end
    //     1609459201, // end
    //     36, // periods
    //   ));
    //   console.log(logs);
    //   // expect(logs).to.contain("range end index 8");
    // });

    // it("fails if wallet mint isn't equal to vesting mint", async () => {
    //   const fakeMint = await createMint(
    //     provider.connection,
    //     payer,
    //     payer.publicKey,
    //     null,
    //     9
    //   );

    //   const fakeWallet = await createAccount(
    //     provider.connection,
    //     payer,
    //     fakeMint,
    //     payer.publicKey
    //   );

    //   await Vesting.init(
    //     {
    //       vesteeWallet: fakeWallet
    //     },
    //     10_000,
    //     1577836801, // start
    //     1609459201, // cliff end
    //     1609459201, // end
    //     36, // periods
    //   );
    //   // console.log(logs);
    //   // expect(logs).to.contain("range end index 8");
    // });

    it.only("works", async () => {
      const vesting = await Vesting.init(
          {
            vesteeWallet,
            mint: vestingMint,
          },
          10_000,
          1577836801, // start
          1609459201, // cliff end
          1609459201, // end
          36, // periods
        )

      const vestingInfo = await vesting.fetch();
      console.log(vestingInfo);
      console.log(vestingInfo.totalVestingAmount.amount.toNumber());

      expect(vestingInfo.totalVestingAmount.amount.toNumber()).to.eq(10_000);
      expect(vestingInfo.startTs.time.toNumber()).to.eq(1577836801);
      expect(vestingInfo.cliffEndTs.time.toNumber()).to.eq(1609459201);
      expect(vestingInfo.endTs.time.toNumber()).to.eq(1609459201);
      expect(vestingInfo.periodCount.toNumber()).to.eq(36);
      expect(vestingInfo.beneficiary).to.deep.eq(vesteeWallet);
      expect(vestingInfo.mint).to.deep.eq(vestingMint);
      expect(vestingInfo.vault).to.deep.eq(await vesting.vestingVault());
      expect(vestingInfo.cumulativeVestedAmount.amount.toNumber()).to.eq(0);
      expect(vestingInfo.cumulativeWithdrawnAmount.amount.toNumber()).to.eq(0);
      expect(vestingInfo.vestingVaultBalance.amount.toNumber()).to.eq(0);
      expect(vestingInfo.unfundedLiabilities.amount.toNumber()).to.eq(0);

    });
  
    // it("fails if farm account already exists", async () => {
    //   const farm = await Farm.init();
  
    //   const logs = await errLogs(Farm.init({ keypair: farm.keypair }));
    //   expect(logs).to.contain("already in use");
    // });
  
    // it("fails if provided with incorrect PDA signer address", async () => {
    //   const logs = await errLogs(
    //     Farm.init({
    //       pda: Keypair.generate().publicKey,
    //     })
    //   );
    //   expect(logs).to.contain("seeds constraint was violated");
    // });
  
    // it("fails if admin isn't signer", async () => {
    //   await expect(Farm.init({ skipAdminSignature: true })).to.be.rejected;
    // });
  
    // it("fails if vesting vault PDA is invalid", async () => {
    //   const logs = await errLogs(
    //     Farm.init({ stakeVault: Keypair.generate().publicKey })
    //   );
    //   expect(logs).to.contain("unauthorized signer");
    // });
  
    // it("fails if vesting wallet is of wrong mint invalid", async () => {
    //   const logs = await errLogs(
    //     Farm.init({ stakeVault: Keypair.generate().publicKey })
    //   );
    //   expect(logs).to.contain("unauthorized signer");
    // });
  
    // it("works", async () => {
    //   const farm = await Farm.init();
    //   const farmInfo = await farm.fetch();
  
    //   expect(farmInfo.admin).to.deep.eq(farm.admin.publicKey);
    //   expect(farmInfo.stakeMint).to.deep.eq(farm.stakeMint);
    //   expect(farmInfo.stakeVault).to.deep.eq(await farm.stakeVault());
  
    //   const stakeVault = await getAccount(
    //     provider.connection,
    //     farmInfo.stakeVault
    //   );
    //   expect(stakeVault.mint).to.deep.eq(farm.stakeMint);
    //   expect(stakeVault.owner).to.deep.eq((await farm.signer())[0]);
    //   expect(stakeVault.closeAuthority).to.eq(null);
    //   expect(stakeVault.isInitialized).to.eq(true);
  
    //   expect(farmInfo.harvests).to.be.lengthOf(10);
    //   (farmInfo.harvests as any[]).forEach((h) => {
    //     expect(h.mint).to.deep.eq(PublicKey.default);
    //     expect(h.vault).to.deep.eq(PublicKey.default);
  
    //     expect(h.periods).to.be.lengthOf(10);
    //     h.periods.forEach(({ tps, startsAt, endsAt }) => {
    //       expect(tps.amount.toNumber()).to.eq(0);
    //       expect(startsAt.slot.toNumber()).to.eq(0);
    //       expect(endsAt.slot.toNumber()).to.eq(0);
    //     });
    //   });
  
    //   expect(farmInfo.snapshots.ringBufferTip.toNumber()).to.eq(0);
    //   expect(farmInfo.snapshots.ringBuffer).to.be.lengthOf(1_000);
    //   (farmInfo.snapshots.ringBuffer as any[]).forEach(
    //     ({ staked, startedAt }) => {
    //       expect(staked.amount.toNumber()).to.eq(0);
    //       expect(startedAt.slot.toNumber()).to.eq(0);
    //     }
    //   );
  
    //   expect(farmInfo.minSnapshotWindowSlots.toNumber()).to.eq(0);
    // });
  });
}
