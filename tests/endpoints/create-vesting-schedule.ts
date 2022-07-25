import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { createMint, createAccount, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { errLogs, provider, payer, getErr } from "../helpers";
import { Vesting } from "../vesting";

export function test() {
  describe("create_vesting_schedule", () => {
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

    it("fails if wallet account isn't initialized", async () => {
      const fakeWallet = Keypair.generate().publicKey;
      const logs = await errLogs(Vesting.init({vesteeWallet: fakeWallet}));
      
      expect(logs).to.contain("AccountNotInitialized.");
    });

    it("fails if wallet mint isn't equal to vesting mint", async () => {
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
        payer.publicKey
      );

      const logs = await errLogs(
        Vesting.init(
          {
            vesteeWallet: fakeWallet,
            mint: vestingMint,
          }
      ));

      expect(logs).to.contain("Vestee wallet must be of correct mint");
    });
  
    it("fails if vesting account already exists", async () => {
      const vesting = await Vesting.init(
        {
          vesteeWallet,
          mint: vestingMint,
        }
      )
  
      const logs = await errLogs(
        Vesting.init({ keypair: vesting.keypair }));

      expect(logs).to.contain("already in use");
    });
  
    it("fails if provided with incorrect PDA signer address", async () => {
      const logs = await errLogs(
        Vesting.init({
          pda: Keypair.generate().publicKey,
        })
      );
      expect(logs).to.contain("seeds constraint was violated");
    });
  
    it("fails if admin isn't signer", async () => {
      const logs = await getErr(
        Vesting.init({ skipAdminSignature: true })
      );

      expect(logs).to.contain("Signature verification failed");
    });

    it("fails if vesting keypair isn't signer", async () => {
      const logs = await getErr(
        Vesting.init({ skipKeypairSignature: true })
      );
      expect(logs).to.contain("Signature verification failed");
    });

    it("works", async () => {
      const adminKeypair = Keypair.generate();
      const vesting = await Vesting.init(
          {
            adminKeypair,
            vesteeWallet,
            mint: vestingMint,
          }
        )

      const vestingInfo = await vesting.fetch();

      // These are the default amounts in the init method
      expect(vestingInfo.totalVestingAmount.amount.toNumber()).to.eq(10_000);
      expect(vestingInfo.startTs.time.toNumber()).to.eq(1577836801);
      expect(vestingInfo.cliffPeriods.toNumber()).to.eq(12);
      expect(vestingInfo.totalPeriods.toNumber()).to.eq(48);
    
      expect(vestingInfo.periodType).to.deep.eq({monthly : {}});

      expect(vestingInfo.admin).to.deep.eq(adminKeypair.publicKey);
      expect(vestingInfo.vesteeWallet).to.deep.eq(vesteeWallet);
      expect(vestingInfo.mint).to.deep.eq(vestingMint);
      expect(vestingInfo.vault).to.deep.eq(await vesting.vestingVault());
      expect(vestingInfo.cumulativeVestedAmount.amount.toNumber()).to.eq(0);
      expect(vestingInfo.cumulativeWithdrawnAmount.amount.toNumber()).to.eq(0);
      expect(vestingInfo.vestingVaultBalance.amount.toNumber()).to.eq(0);
      expect(vestingInfo.unfundedLiabilities.amount.toNumber()).to.eq(0);

    });
  });
}
