import {
  NATIVE_MINT,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createAccount,
  getAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { PublicKey, Keypair } from "@solana/web3.js";
import { QUOTE_MINT, memechan, payer, provider } from "./helpers";
import { AmmPool } from "./pool";
import { Address, BN } from "@coral-xyz/anchor";
import { MemeTicketWrapper } from "./ticket";
import { raydiumProgram } from "./raydium/raydium";
import { MEMO_PROGRAM_ID } from "@raydium-io/raydium-sdk";
import { StakingPool } from "./sol-sdk/staking-pool/StakingPool";
import { getAuthAddress } from "./raydium/utils";

export interface UnstakeArgs {
  ticket: MemeTicketWrapper;
  amount: BN;
  user: Keypair;
}

export interface WithdrawFeesArgs {
  ticket: MemeTicketWrapper;
  user: Keypair;
}

export class StakingWrapper {
  public constructor(public id: PublicKey) {
    //
  }

  public async fetch() {
    return memechan.account.stakingPool.fetch(this.id);
  }

  public static findSignerPda(
    publicKey: PublicKey,
    memechanProgramId: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("staking"), publicKey.toBytes()],
      memechanProgramId
    )[0];
  }

  public static signerFrom(publicKey: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("staking"), publicKey.toBytes()],
      memechan.programId
    )[0];
  }

  public async add_fees(ammPool: AmmPool) {
    const amm = await raydiumProgram.account.poolState.fetch(ammPool.id);
    const staking = await memechan.account.stakingPool.fetch(this.id);

    const [auth] = await getAuthAddress(raydiumProgram.programId);

    await memechan.methods
      .addFees()
      .accounts({
        signer: payer.publicKey,
        staking: this.id,
        stakingLpWallet: staking.lpVault,
        stakingSignerPda: StakingPool.findSignerPda(
          this.id,
          memechan.programId
        ),
        memeMint: ammPool.memeMint,
        quoteMint: ammPool.quoteMint,
        memeVault: staking.memeVault,
        quoteVault: staking.quoteVault,
        raydiumAmm: ammPool.id,
        raydiumAmmAuthority: auth,
        raydiumLpMint: amm.lpMint,
        raydiumMemeVault: amm.token0Vault,
        raydiumQuoteVault: amm.token1Vault,
        raydiumProgram: raydiumProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenProgram22: TOKEN_2022_PROGRAM_ID,
        memoProgram: MEMO_PROGRAM_ID,
      })
      .signers([payer])
      .rpc();
  }

  public signer(): PublicKey {
    return StakingWrapper.signerFrom(this.id);
  }

  public signerPda(): PublicKey {
    return StakingWrapper.signerFrom(this.id);
  }

  public async unstake(input: UnstakeArgs): Promise<[PublicKey, PublicKey]> {
    let user = input.user;

    let stakingInfo = await this.fetch();

    const memeAccKey = Keypair.generate();
    const memeAcc = await createAccount(
      provider.connection,
      user,
      stakingInfo.memeMint,
      user.publicKey,
      memeAccKey
    );

    const wsolAccKey = Keypair.generate();
    const quoteAcc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      QUOTE_MINT,
      user.publicKey
    );

    await memechan.methods
      .unstake(input.amount)
      .accounts({
        memeTicket: input.ticket.id,
        signer: input.user.publicKey,
        stakingSignerPda: this.signer(),
        memeVault: stakingInfo.memeVault,
        quoteVault: stakingInfo.quoteVault,
        staking: this.id,
        userMeme: memeAcc,
        userQuote: quoteAcc.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    return [memeAcc, quoteAcc.address];
  }

  public async withdraw_fees(
    input: WithdrawFeesArgs
  ): Promise<[PublicKey, PublicKey]> {
    let user = input.user;

    let stakingInfo = await this.fetch();

    const memeAccKey = Keypair.generate();
    const memeAcc = await createAccount(
      provider.connection,
      user,
      stakingInfo.memeMint,
      user.publicKey,
      memeAccKey
    );

    const wsolAccKey = Keypair.generate();
    const quoteAcc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      QUOTE_MINT,
      user.publicKey
    );

    await memechan.methods
      .withdrawFees()
      .accounts({
        memeTicket: input.ticket.id,
        stakingSignerPda: this.signer(),
        memeVault: stakingInfo.memeVault,
        quoteVault: stakingInfo.quoteVault,
        staking: this.id,
        userMeme: memeAcc,
        userQuote: quoteAcc.address,
        signer: user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    return [memeAcc, quoteAcc.address];
  }
}
