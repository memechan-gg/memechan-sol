import {
  NATIVE_MINT,
  TOKEN_PROGRAM_ID,
  createAccount,
  getAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import {
  PublicKey,
  Keypair
} from "@solana/web3.js";
import { QUOTE_MINT, memechan, payer, provider } from "./helpers";
import { AmmPool } from "./pool";
import { Address, BN } from "@coral-xyz/anchor";
import { MemeTicket } from "./ticket";

export interface UnstakeArgs {
  ticket: MemeTicket;
  amount: BN;
  user: Keypair;
}

export interface WithdrawFeesArgs {
  ticket: MemeTicket;
  user: Keypair;
}

export class Staking {
  public constructor(public id: PublicKey) {
    //
  }


  public async fetch() {
    return memechan.account.stakingPool.fetch(this.id);
  }

  public static findSignerPda(publicKey: PublicKey, memechanProgramId: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync([Buffer.from("staking"), publicKey.toBytes()], memechanProgramId)[0];
  }
  
  public static signerFrom(publicKey: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("staking"), publicKey.toBytes()],
      memechan.programId
    )[0];
  }

  public async add_fees(ammPool: AmmPool) {

    const info = await this.fetch();

    const pda = this.signerPda()
    const staking = this.id;

   // const ammInfo = await ammPool.fetch()

    const getAccountMetaFromPublicKey = (pk) => {
      return { isSigner: false, isWritable: true, pubkey: pk };
    };

    // await memechan.methods.addFees()
    //   .accounts({
    //     memeVault: info.memeVault,
    //     wsolVault: info.wsolVault,
    //     staking,
    //     aldrinPoolAcc: ammPool.id,
    //     aldrinAmmProgram: amm.programId,
    //     aldrinLpMint: ammInfo.mint,
    //     aldrinPoolLpWallet: ammInfo.programTollWallet,
    //     aldrinPoolSigner: ammPool.signer(),
    //     stakingSignerPda: this.signer(),
    //     tokenProgram: TOKEN_PROGRAM_ID,
    //   })
    //   .remainingAccounts([
    //     getAccountMetaFromPublicKey(ammInfo.reserves[0].vault),
    //     getAccountMetaFromPublicKey(ammInfo.reserves[1].vault)
    //   ])
    //   .signers([payer])
    //   .rpc();
  }

  public signer(): PublicKey {
    return Staking.signerFrom(this.id);
  }

  public signerPda(): PublicKey {
    return Staking.signerFrom(this.id);
  }

  public async unstake(
    input: UnstakeArgs
  ): Promise<[PublicKey, PublicKey]> {

    let user = input.user;

    let stakingInfo = await this.fetch()

    const memeAccKey = Keypair.generate();
    const memeAcc = await createAccount(
      provider.connection,
      user,
      stakingInfo.memeMint,
      user.publicKey,
      memeAccKey
    );

    const wsolAccKey = Keypair.generate();
    const quoteAcc = await getOrCreateAssociatedTokenAccount(provider.connection, user, QUOTE_MINT, user.publicKey)

    await memechan.methods.unstake(input.amount)
      .accounts({
        memeTicket: input.ticket.id,
        signer: input.user.publicKey,
        stakingSignerPda: this.signer(),
        memeVault: stakingInfo.memeVault,
        quoteVault: stakingInfo.quoteVault,
        staking: this.id,
        userMeme: memeAcc,
        userQuote: quoteAcc.address,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([user])
      .rpc();

    return [memeAcc, quoteAcc.address];
  }

  public async withdraw_fees(
    input: WithdrawFeesArgs
  ): Promise<[PublicKey, PublicKey]> {

    let user = input.user;

    let stakingInfo = await this.fetch()

    const memeAccKey = Keypair.generate();
    const memeAcc = await createAccount(
      provider.connection,
      user,
      stakingInfo.memeMint,
      user.publicKey,
      memeAccKey
    );

    const wsolAccKey = Keypair.generate();
    const quoteAcc = await getOrCreateAssociatedTokenAccount(provider.connection, user, QUOTE_MINT, user.publicKey)

    await memechan.methods.withdrawFees()
      .accounts({
        memeTicket: input.ticket.id,
        stakingSignerPda: this.signer(),
        memeVault: stakingInfo.memeVault,
        quoteVault: stakingInfo.quoteVault,
        staking: this.id,
        userMeme: memeAcc,
        userQuote: quoteAcc.address,
        signer: user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([user])
      .rpc();

    return [memeAcc, quoteAcc.address];
  }
}
