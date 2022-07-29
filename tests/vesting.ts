import { vesting, payer, provider, airdrop } from "./helpers";
import { Keypair, PublicKey, SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js";
import {
  createAccount,
  createMint,
  mintTo,
  Account,
  getAccount,
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
  vestingAmount: number;
  startTs: number;
  cliffPeriods: number;
  totalPeriods: number;
  periodType: number;
}

export interface ChangeVesteeWalletArgs {
  adminKeypair: Keypair;
  vestingKeypair: Keypair;
  vesteeWalletNew: PublicKey;
  skipAdminSignature: boolean;
  skipCreateVesting: boolean;
}

export interface UpdateVestedTokensArgs {
  vestingKeypair: Keypair;
}

export interface FundVestingVault{
  vestingKeypair: Keypair;
  walletAuthority: Keypair;
  vestingVault: PublicKey;
  fundingWallet: PublicKey;
  skipAuthoritySignature: boolean,
}

export interface WithdrawVestedTokens{
  vestingKeypair: Keypair;
  vestingVault: PublicKey;
  pda: PublicKey;
  vesteeWallet: PublicKey;
}

export interface CloseVestingSchedule{
  adminKeypair: Keypair;
  vestingKeypair: Keypair;
  skipAdminSignature: boolean;
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
    ): Promise<Vesting> {
    const adminKeypair = input.adminKeypair ?? Keypair.generate();
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


    const vestingAmount = input.vestingAmount ?? 10_000;
    const startTs = input.startTs ?? 1577836801; // Jan 01 2020
    const cliffPeriods = input.cliffPeriods ?? 12;
    const totalPeriods = input.totalPeriods ?? 48;
    const periodType = input.periodType ?? 2; // Monthly

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
        {time: new BN(startTs)},
        new BN(cliffPeriods),
        new BN(totalPeriods),
        periodType,
      )
      .accounts({
        admin: adminKeypair.publicKey,
        vesting: vestingKeypair.publicKey,
        vestingSigner: vestingSignerPda,
        mint,
        vestingVault,
        vesteeWallet,
        tokenProgram: TOKEN_PROGRAM_ID,
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

  public async changeVesteeWallet(
    input: Partial<ChangeVesteeWalletArgs> = {},
    ) {
    const adminKeypair = input.adminKeypair ?? Keypair.generate();
    await airdrop(adminKeypair.publicKey);

    const vestingKeypair = input.vestingKeypair ?? this.keypair;
    const skipAdminSignature = input.skipAdminSignature ?? false;
    const skipCreateVesting = input.skipCreateVesting ?? false;

    const vesteeWalletNew = input.vesteeWalletNew ??
      (await (async () => {
        const wallet = await createAccount(
          provider.connection,
          payer,
          this.mint,
          payer.publicKey
        )
        return wallet;
    })());

    const preInstructions = [];
    const signers = [];
    if (!skipAdminSignature) {
      signers.push(adminKeypair);
    }

    await vesting.methods
      .changeVesteeWallet()
      .accounts({
        admin: adminKeypair.publicKey,
        vesting: vestingKeypair.publicKey,
        vesteeWalletNew,
      })
      .signers(signers)
      .preInstructions(preInstructions)
      .rpc();
  }

  public async updateVestedTokens(
    input: Partial<UpdateVestedTokensArgs> = {},
    ) {
    const vestingKeypair = input.vestingKeypair ?? this.keypair;

    await vesting.methods
      .updateVestedTokens()
      .accounts({
        vesting: vestingKeypair.publicKey,
        clock: SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();
  }

  public async fundVestingVault(
    input: Partial<FundVestingVault> = {},
    fundingAmount: number,
    ) {
    const vestingKeypair = input.vestingKeypair ?? this.keypair;
    const walletAuthority = input.walletAuthority ?? Keypair.generate();
    const vestingVault = input.vestingVault ?? await this.vestingVault();
    const fundingWallet = input.fundingWallet ??
      (await createAccount(provider.connection, payer, this.mint, walletAuthority.publicKey));
    const skipAuthoritySignature = input.skipAuthoritySignature ?? false;

      const signers = [];
      if (!skipAuthoritySignature) {
        signers.push(walletAuthority);
      }

    await vesting.methods
      .fundVestingVault({amount: new BN(fundingAmount)})
      .accounts({
        vesting: vestingKeypair.publicKey,
        vestingVault,
        walletAuthority: walletAuthority.publicKey,
        fundingWallet,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers(signers)
      .rpc();
  }

  public async withdrawVestedTokens(
    input: Partial<WithdrawVestedTokens> = {},
    withdrawAmount: number,
  ) {
    const vestingKeypair = input.vestingKeypair ?? this.keypair;
    const vestingVault = input.vestingVault ?? await this.vestingVault();
    const vestingSignerPda =
    input.pda ??
    (await (async () => {
      const [pda, _] = await Vesting.signerFrom(vestingKeypair.publicKey);
      return pda;
    })());

    const vesteeWallet = input.vesteeWallet ??
      (await (async () => {
        const wallet = await createAccount(
          provider.connection,
          payer,
          this.mint,
          payer.publicKey
        )
        return wallet;
    })());

    await vesting.methods
      .withdrawVestedTokens({amount: new BN(withdrawAmount)})
      .accounts({
        vesting: vestingKeypair.publicKey,
        vestingVault,
        vestingSigner: vestingSignerPda,
        vesteeWallet,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  }

  public async closeVestingSchedule(
    input: Partial<CloseVestingSchedule> = {},
    ) {
    const adminKeypair = input.adminKeypair ?? Keypair.generate();
    await airdrop(adminKeypair.publicKey);

    const vestingKeypair = input.vestingKeypair ?? this.keypair;
    const skipAdminSignature = input.skipAdminSignature ?? false;

    const signers = [];
    if (!skipAdminSignature) {
      signers.push(adminKeypair);
    }

    await vesting.methods
      .closeVestingSchedule()
      .accounts({
        admin: adminKeypair.publicKey,
        vesting: vestingKeypair.publicKey,
      })
      .signers(signers)
      .rpc();
  }
}
