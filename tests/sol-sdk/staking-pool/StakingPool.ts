import { Program } from "@coral-xyz/anchor";
import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { AccountMeta, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { MemechanClient } from "../MemechanClient";
import { BoundPoolClient } from "../bound-pool/BoundPool";
import { MemeTicket, MemeTicketFields } from "../memeticket/MemeTicket";
import {
  AddFeesArgs,
  GetAddFeesTransactionArgs,
  GetUnstakeTransactionArgs,
  GetWithdrawFeesTransactionArgs,
  UnstakeArgs,
  WithdrawFeesArgs,
} from "./types";
import { getSendAndConfirmTransactionMethod } from "../util/getSendAndConfirmTransactionMethod";
import { getCreateTokenAccountInstructions } from "../util/getCreateAccountInstruction";
import { retry } from "../util/retry";
import { MEMECHAN_QUOTE_MINT } from "../config/config";
import { formatAmmKeysById } from "../raydium/formatAmmKeysById";
import { MEMO_PROGRAM_ID, RAYDIUM_MAINNET } from "@raydium-io/raydium-sdk";
import { PROGRAMIDS } from "../raydium/config";
import { MemechanSol } from "../../../target/types/memechan_sol";
import { QUOTE_MINT } from "../../helpers";
import { CPMM } from "../../common";

export class StakingPool {
  constructor(
    public id: PublicKey,
    private client: MemechanClient,
    public pool: PublicKey,
    public memeVault: PublicKey,
    public memeMint: PublicKey,
    public lpVault: PublicKey,
    public lpMint: PublicKey,
    public quote_vault: PublicKey,
    public amm: PublicKey
  ) {}

  public static async fromStakingPoolId({
    client,
    poolAccountAddressId,
  }: {
    client: MemechanClient;
    poolAccountAddressId: PublicKey;
  }) {
    const stakingPoolObjectData =
      await client.memechanProgram.account.stakingPool.fetch(
        poolAccountAddressId
      );

    console.log("stakingPoolObjectData:", stakingPoolObjectData);

    const boundClientInstance = new StakingPool(
      poolAccountAddressId,
      client,
      stakingPoolObjectData.pool,
      stakingPoolObjectData.memeVault,
      stakingPoolObjectData.memeMint,
      stakingPoolObjectData.lpVault,
      stakingPoolObjectData.lpMint,
      stakingPoolObjectData.quoteVault,
      stakingPoolObjectData.ammPool
    );

    return boundClientInstance;
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

  public async getAddFeesTransaction({
    transaction,
    ammPoolId,
    payer,
  }: GetAddFeesTransactionArgs): Promise<{ tx: Transaction }> {
    const tx = transaction ?? new Transaction();
    const stakingInfo = await this.fetch();

    const ammPool = await formatAmmKeysById(
      ammPoolId.toBase58(),
      this.client.connection
    );

    const stakingSignerPda = this.findSignerPda();
    //const stakingLpWalletKeypair = Keypair.generate();
    const stakingLpWallet = stakingInfo.lpVault;

    // const createStakingLpWalletInstructions = await getCreateAccountInstructions(
    //   this.client.connection,
    //   payer.publicKey,
    //   stakingInfo.lpMint,
    //   stakingSignerPda,
    //   stakingLpWalletKeypair,
    //   "confirmed",
    //   this.client.memechanProgram.programId,
    // );

    // tx.add(...createStakingLpWalletInstructions);

    const addFeesInstruction = await this.client.memechanProgram.methods
      .addFees()
      .accounts({
        memeVault: stakingInfo.memeVault,
        memeMint: stakingInfo.memeMint,
        quoteVault: stakingInfo.quoteVault,
        quoteMint: QUOTE_MINT,
        staking: this.id,
        stakingSignerPda: stakingSignerPda,
        raydiumAmm: ammPool.id,
        raydiumAmmAuthority: ammPool.authority,
        raydiumLpMint: ammPool.lpMint,
        raydiumMemeVault: ammPool.baseVault,
        raydiumQuoteVault: ammPool.quoteVault,
        raydiumProgram: CPMM,
        signer: payer.publicKey,
        stakingLpWallet: stakingLpWallet,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenProgram22: TOKEN_2022_PROGRAM_ID,
        memoProgram: MEMO_PROGRAM_ID,
      })
      .instruction();

    tx.add(addFeesInstruction);

    return { tx };
  }

  public async addFees({
    payer,
    transaction,
    ammPoolId,
  }: AddFeesArgs): Promise<void> {
    const { tx: addFeesTransaction } = await this.getAddFeesTransaction({
      transaction,
      ammPoolId,
      payer,
    });

    const sendAndConfirmAddFeesTransaction = getSendAndConfirmTransactionMethod(
      {
        connection: this.client.connection,
        signers: [payer],
        transaction: addFeesTransaction,
      }
    );

    await retry({
      fn: sendAndConfirmAddFeesTransaction,
      functionName: "addFees",
    });
  }

  public async getUnstakeTransaction(args: GetUnstakeTransactionArgs): Promise<{
    transaction: Transaction;
    memeAccountKeypair: Keypair;
    quoteAccountKeypair: Keypair;
  }> {
    const tx = args.transaction ?? new Transaction();
    const stakingInfo = await this.fetch();

    const memeAccountKeypair = Keypair.generate();
    const memeAccountPublicKey = memeAccountKeypair.publicKey;
    const createMemeAccountInstructions =
      await getCreateTokenAccountInstructions(
        this.client.connection,
        args.user.publicKey,
        stakingInfo.memeMint,
        args.user.publicKey,
        memeAccountKeypair
      );

    tx.add(...createMemeAccountInstructions);

    const quoteAccountKeypair = Keypair.generate();
    const quoteAccountPublicKey = quoteAccountKeypair.publicKey;
    const createQuoteAccountInstructions =
      await getCreateTokenAccountInstructions(
        this.client.connection,
        args.user.publicKey,
        MEMECHAN_QUOTE_MINT,
        args.user.publicKey,
        quoteAccountKeypair
      );

    tx.add(...createQuoteAccountInstructions);

    const unstakeInstruction = await this.client.memechanProgram.methods
      .unstake(args.amount)
      .accounts({
        memeTicket: args.ticket.id,
        signer: args.user.publicKey,
        stakingSignerPda: this.findSignerPda(),
        memeVault: stakingInfo.memeVault,
        quoteVault: stakingInfo.quoteVault,
        staking: this.id,
        userMeme: memeAccountPublicKey,
        userQuote: quoteAccountPublicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    tx.add(unstakeInstruction);

    return { transaction: tx, memeAccountKeypair, quoteAccountKeypair };
  }

  public async unstake(args: UnstakeArgs): Promise<{
    memeAccountPublicKey: PublicKey;
    quoteAccountPublicKey: PublicKey;
  }> {
    const { memeAccountKeypair, transaction, quoteAccountKeypair } =
      await this.getUnstakeTransaction(args);

    const sendAndConfirmUnstakeTransaction = getSendAndConfirmTransactionMethod(
      {
        connection: this.client.connection,
        signers: [args.user, memeAccountKeypair, quoteAccountKeypair],
        transaction,
      }
    );

    await retry({
      fn: sendAndConfirmUnstakeTransaction,
      functionName: "unstake",
    });

    return {
      memeAccountPublicKey: memeAccountKeypair.publicKey,
      quoteAccountPublicKey: quoteAccountKeypair.publicKey,
    };
  }

  public async getWithdrawFeesTransaction(
    args: GetWithdrawFeesTransactionArgs
  ): Promise<{
    transaction: Transaction;
    memeAccountKeypair: Keypair;
    quoteAccountKeypair: Keypair;
  }> {
    const tx = args.transaction ?? new Transaction();
    const stakingInfo = await this.fetch();

    const memeAccountKeypair = Keypair.generate();
    const memeAccountPublicKey = memeAccountKeypair.publicKey;
    const createMemeAccountInstructions =
      await getCreateTokenAccountInstructions(
        this.client.connection,
        args.user.publicKey,
        stakingInfo.memeMint,
        args.user.publicKey,
        memeAccountKeypair
      );

    tx.add(...createMemeAccountInstructions);

    const quoteAccountKeypair = Keypair.generate();
    const quoteAccountPublicKey = quoteAccountKeypair.publicKey;
    const createWSolAccountInstructions =
      await getCreateTokenAccountInstructions(
        this.client.connection,
        args.user.publicKey,
        MEMECHAN_QUOTE_MINT,
        args.user.publicKey,
        quoteAccountKeypair
      );

    tx.add(...createWSolAccountInstructions);

    const withdrawFeesInstruction = await this.client.memechanProgram.methods
      .withdrawFees()
      .accounts({
        memeTicket: args.ticket.id,
        stakingSignerPda: this.findSignerPda(),
        memeVault: stakingInfo.memeVault,
        quoteVault: stakingInfo.quoteVault,
        staking: this.id,
        userMeme: memeAccountPublicKey,
        userQuote: quoteAccountPublicKey,
        signer: args.user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    tx.add(withdrawFeesInstruction);

    return { transaction: tx, memeAccountKeypair, quoteAccountKeypair };
  }

  public async withdrawFees(args: WithdrawFeesArgs): Promise<{
    memeAccountPublicKey: PublicKey;
    quoteAccountPublicKey: PublicKey;
  }> {
    const { memeAccountKeypair, transaction, quoteAccountKeypair } =
      await this.getWithdrawFeesTransaction(args);

    const sendAndConfirmWithdrawFeesTransaction =
      getSendAndConfirmTransactionMethod({
        connection: this.client.connection,
        signers: [args.user, memeAccountKeypair, quoteAccountKeypair],
        transaction,
      });

    await retry({
      fn: sendAndConfirmWithdrawFeesTransaction,
      functionName: "withdrawFees",
    });

    return {
      memeAccountPublicKey: memeAccountKeypair.publicKey,
      quoteAccountPublicKey: quoteAccountKeypair.publicKey,
    };
  }

  public async getHoldersCount() {
    return StakingPool.getHoldersCount(this.pool, this.memeMint, this.client);
  }

  public async getHoldersList() {
    return StakingPool.getHoldersList(this.pool, this.memeMint, this.client);
  }

  /**
   * Fetches all tickets for corresponding pool id
   */
  public async fetchRelatedTickets(
    pool = this.pool,
    client = this.client
  ): Promise<MemeTicketFields[]> {
    return MemeTicket.fetchRelatedTickets(pool, client);
  }

  /**
   * Fetches all unique token holders and memetickets owners for pool; then returns their number
   */
  public static async getHoldersCount(
    pool: PublicKey,
    mint: PublicKey,
    client: MemechanClient
  ) {
    return (await StakingPool.getHoldersList(pool, mint, client)).length;
  }

  /**
   * Fetches all unique token holders and memetickets owners for pool; then returns thier addresses
   */
  public static async getHoldersList(
    pool: PublicKey,
    mint: PublicKey,
    client: MemechanClient
  ) {
    const ticketHolderList = await BoundPoolClient.getHoldersList(pool, client);
    const tokenHolderList = await StakingPool.getTokenHolderListHelius(
      mint,
      client.heliusApiUrl
    );

    return Array.from(tokenHolderList);
  }

  public static async getTokenHolderListHelius(mint: PublicKey, url: string) {
    return [];
  }

  private async fetch(program = this.client.memechanProgram) {
    return program.account.stakingPool.fetch(this.id);
  }

  public static async all(program: Program<MemechanSol>) {
    return program.account.stakingPool.all();
  }

  public findSignerPda(): PublicKey {
    return StakingPool.findSignerPda(
      this.id,
      this.client.memechanProgram.programId
    );
  }

  private getAccountMeta(pubkey: PublicKey): AccountMeta {
    return { isSigner: false, isWritable: true, pubkey };
  }
}
