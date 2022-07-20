import { vesting, payer, provider } from "./helpers";
import { Keypair, PublicKey } from "@solana/web3.js";
import {
  createAccount,
  createMint,
  transfer,
  mintTo,
  Account,
  getAccount,
  getOrCreateAssociatedTokenAccount,
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
