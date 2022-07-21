import { vesting, payer, provider, airdrop } from "./helpers";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  createAccount,
  createMint,
  transfer,
  mintTo,
  Account,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { BN } from "@project-serum/anchor";

export interface InitVestingArgs {
  adminKeypair: Keypair;
  keypair: Keypair;
  pda: PublicKey;
  vestingVault: PublicKey;
  vesteeWallet: PublicKey;
  mint: PublicKey;
  skipAdminSignature: boolean;
  skipKeypairSignature: boolean;
  skipCreateVesting: boolean;
}


export class Vesting {
  public get id(): PublicKey {
    return this.keypair.publicKey;
  }

  private constructor(
    public keypair: Keypair,
    public admin: Keypair,
    public mint: PublicKey
  ) {
    //
  }

  public static async init(
    input: Partial<InitVestingArgs> = {},
    vestingAmount: number,
    startTs: number,
    cliffEndTs: number,
    endTs: number,
    periodCount: number,
    ): Promise<Vesting> {
    const adminKeypair = input.adminKeypair ?? payer;
    await airdrop(adminKeypair.publicKey);
    const vestingKeypair = input.keypair ?? Keypair.generate();
    const skipAdminSignature = input.skipAdminSignature ?? false;
    const skipKeypairSignature = input.skipKeypairSignature ?? false;
    const skipCreateVesting = input.skipCreateVesting ?? false;

    const vestingSignerPda =
      input.pda ??
      (await (async () => {
        const [pda, _] = await Vesting.signerFrom(vestingKeypair.publicKey);
        return pda;
      })());

    const mint = 
      input.mint ?? 
      (await (async () => {
      return createMint(
        provider.connection,
        payer,
        adminKeypair.publicKey,
        null,
        6
      );
    })());

    const vestingVault =
      input.vestingVault ??
      (await (async () => {
        const [pda, _bumpSeed] = PublicKey.findProgramAddressSync(
          [Buffer.from("vault"), vestingKeypair.publicKey.toBytes()],
          vesting.programId
        );
        return pda;
    })());

    const vestingVault2 = await createAccount(
      provider.connection,
      payer,
      mint,
      Keypair.generate().publicKey
    );
      
    const vesteeWallet = input.vesteeWallet ?? 
      (await (async () => {
        const wallet = await createAccount(
          provider.connection,
          payer,
          mint,
          payer.publicKey
        )
        return wallet;
    })());

    
    const preInstructions = [];
    if (!skipCreateVesting) {
      preInstructions.push(
        await vesting.account.vesting.createInstruction(vestingKeypair)
      );
    }
    
    const signers = [];
    if (!skipAdminSignature) {
      signers.push(adminKeypair);
    }

    if (!skipKeypairSignature) {
      signers.push(vestingKeypair);
    }

    await vesting.methods
      .createVestingSchedule(
        {amount: new BN(vestingAmount)},
        new BN(startTs),
        new BN(cliffEndTs),
        new BN(endTs),
        new BN(periodCount),
      )
      .accounts({
        admin: adminKeypair.publicKey,
        vesting: vestingKeypair.publicKey,
        vestingSigner: vestingSignerPda,
        mint,
        vestingVault: vestingVault,
        vesteeWallet,
        tokenProgram: TOKEN_PROGRAM_ID,
        // systemProgram: SystemProgram.programId,
      })
      .signers(signers)
      .preInstructions(preInstructions)
      .rpc();

    return new Vesting(vestingKeypair, adminKeypair, mint);
  }

  public async fetch() {
    return vesting.account.vesting.fetch(this.id);
  }

  public async vestingVault(): Promise<PublicKey> {
    const [pda, _bumpSeed] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), this.id.toBytes()],
      vesting.programId
    );
    return pda;
  }

  public static async signerFrom(
    publicKey: PublicKey
  ): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from("signer"), publicKey.toBytes()],
      vesting.programId
    );
  }

  public async signer(): Promise<[PublicKey, number]> {
    return Vesting.signerFrom(this.id);
  }

  public async signerPda(): Promise<PublicKey> {
    const [pda, _] = await Vesting.signerFrom(this.id);
    return pda;
  }

  public async vestingVaultInfo(): Promise<Account> {
    return getAccount(provider.connection, await this.vestingVault());
  }

  public async airdropVestTokens(
    wallet: PublicKey,
    amount: number = 1_000_000
  ) {
    return mintTo(
      provider.connection,
      payer,
      this.mint,
      wallet,
      this.admin,
      amount
    );
  }

}
