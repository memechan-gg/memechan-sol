import {
  ApiPoolInfoV4,
  METADATA_PROGRAM_ID,
  Token,
} from "@raydium-io/raydium-sdk";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createAccount,
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
} from "@solana/spl-token";
import { TokenInfo } from "@solana/spl-token-registry";
import {
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  SYSVAR_CLOCK_PUBKEY,
  SYSVAR_RENT_PUBKEY,
  Signer,
  SystemProgram,
  Transaction,
  TransactionMessage,
  VersionedTransaction,
  sendAndConfirmRawTransaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as solana from "@solana/web3.js";
import BigNumber from "bignumber.js";
import { ensureAssociatedTokenAccountWithIX } from "../util/ensureAssociatedTokenAccountWithIX";
import { AnchorError, BN, Program, Provider } from "@coral-xyz/anchor";
import { MemechanClient } from "../MemechanClient";
import { MemeTicket, MemeTicketFields } from "../memeticket/MemeTicket";
import { StakingPool } from "../staking-pool/StakingPool";
import {
  BoundPoolArgs,
  BuyMemeArgs,
  GetBuyMemeTransactionArgs,
  GetBuyMemeTransactionOutput,
  GetCreateNewBondingPoolAndTokenTransactionArgs,
  GetGoLiveTransactionArgs,
  GetInitChanPoolTransactionArgs,
  GetInitStakingPoolTransactionArgs,
  GetOutputAmountForBuyMeme,
  GetSellMemeTransactionArgs,
  GoLiveArgs,
  InitChanAmmPool,
  InitStakingPoolArgs,
  InitStakingPoolResult,
  SwapXArgs,
  SwapYArgs,
} from "./types";

import { findProgramAddress } from "../common/helpers";
import {
  CHAN_TOKEN_INFO,
  DEFAULT_MAX_M,
  DEFAULT_MAX_M_LP,
  MEMECHAN_MEME_TOKEN_DECIMALS,
  MEMECHAN_QUOTE_TOKEN,
  MEMECHAN_QUOTE_TOKEN_DECIMALS,
  memechan,
  ADMIN_PUB_KEY,
} from "../config/config";
import { formatAmmKeysById } from "../raydium/formatAmmKeysById";

import {
  createMetadata,
  getCreateMetadataTransaction,
} from "../token/createMetadata";
import { createMintWithPriority } from "../token/createMintWithPriority";
import { getCreateMintWithPriorityTransaction } from "../token/getCreateMintWithPriorityTransaction";
import { NewBPInstructionParsed } from "../tx-parsing/parsers/bonding-pool-creation-parser";
import { ParseTx } from "../tx-parsing/parsing";
import { sendTx } from "../util";
import { getTxSize } from "../util/get-tx-size";
import {
  getCreateAssociatedTokenAccountInstructions,
  getCreateTokenAccountInstructions,
} from "../util/getCreateAccountInstruction";
import { getSendAndConfirmTransactionMethod } from "../util/getSendAndConfirmTransactionMethod";
import { retry } from "../util/retry";
import { deductSlippage } from "../util/trading/deductSlippage";
import { normalizeInputCoinAmount } from "../util/trading/normalizeInputCoinAmount";
import { TargetConfig } from "../targetconfig/TargetConfig";
import {
  LUT2SLOT,
  LUTSLOT,
  QUOTE_MINT,
  admin,
  adminSigner,
  getLUTPDA,
  provider,
  sleep,
} from "../../helpers";
import { MemechanSol } from "../../../target/types/memechan_sol";
import { BoundPoolType } from "../../bound_pool";
import {
  BP_FEE_VAULT_OWNER,
  LP_FEE_VAULT_OWNER,
  pointsMint,
  pointsPda,
  pointsAcc,
  SWAP_FEE_VAULT_OWNER,
} from "../../common";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import AmmImpl, {
  LockEscrow,
  VaultIdl,
  derivePoolAddress,
} from "@mercurial-finance/dynamic-amm-sdk";
import {
  calculateUnclaimedLockEscrowFee,
  createProgram,
  deriveLockEscrowPda,
  deriveMintMetadata,
  generateCurveType,
  getAssociatedTokenAccount,
  getOrCreateATAInstruction,
  wrapSOLInstruction,
} from "@mercurial-finance/dynamic-amm-sdk/dist/cjs/src/amm/utils";
import VaultImpl, { getVaultPdas } from "@mercurial-finance/vault-sdk";
import {
  FEE_OWNER,
  SEEDS,
} from "@mercurial-finance/dynamic-amm-sdk/dist/cjs/src/amm/constants";
import { ATA_PROGRAM_ID } from "../raydium/config";
import { LockEscrowAccount } from "@mercurial-finance/dynamic-amm-sdk/dist/cjs/src/amm/types";
import { ChanSwapWrapper } from "../../chan_swap";
import { createAssociatedTokenAccount } from "@solana/spl-token";
import { createAssociatedTokenAccountIdempotent } from "@solana/spl-token";
import { createAssociatedTokenAccountIdempotentInstruction } from "@solana/spl-token";
import { PointsEpochWrapper } from "../../points_epoch";

export class BoundPoolClient {
  private constructor(
    public id: PublicKey,
    public client: MemechanClient,
    public memeVault: PublicKey,
    public quoteVault: PublicKey,
    public memeTokenMint: PublicKey,
    public quoteTokenMint: PublicKey,
    public memeToken: Token,
    public poolInfo: BoundPoolType
  ) {
    //
  }

  public static async fromBoundPoolId({
    client,
    poolAccountAddressId,
  }: {
    client: MemechanClient;
    poolAccountAddressId: PublicKey;
  }) {
    const poolObjectData = await BoundPoolClient.fetch2(
      client.connection,
      poolAccountAddressId
    );

    const boundClientInstance = new BoundPoolClient(
      poolAccountAddressId,
      client,
      poolObjectData.memeReserve.vault,
      poolObjectData.quoteReserve.vault,
      poolObjectData.memeReserve.mint,
      poolObjectData.quoteReserve.mint,
      new Token(
        TOKEN_PROGRAM_ID,
        poolObjectData.memeReserve.mint,
        MEMECHAN_MEME_TOKEN_DECIMALS
      ),
      poolObjectData
    );

    return boundClientInstance;
  }

  public static async fromPoolCreationTransaction({
    client,
    poolCreationSignature,
  }: {
    client: MemechanClient;
    poolCreationSignature: string;
  }) {
    const parsedData = await ParseTx(poolCreationSignature, client);
    console.debug("parsedData: ", parsedData);

    if (!parsedData) {
      throw new Error(
        `No such pool found for such signature ${poolCreationSignature}`
      );
    }

    const newPoolInstructionData = parsedData.find(
      (el): el is NewBPInstructionParsed => el.type === "new_pool"
    );

    if (!newPoolInstructionData) {
      throw new Error(
        `No such pool found in instruction data for signature ${poolCreationSignature}`
      );
    }

    const poolObjectData = await BoundPoolClient.fetch2(
      client.connection,
      newPoolInstructionData.poolAddr
    );

    const boundClientInstance = new BoundPoolClient(
      newPoolInstructionData.poolAddr,
      client,
      poolObjectData.memeReserve.vault,
      poolObjectData.quoteReserve.vault,
      poolObjectData.memeReserve.mint,
      poolObjectData.quoteReserve.mint,
      new Token(TOKEN_PROGRAM_ID, poolObjectData.memeReserve.mint, 6), // TODO fix 6 decimals
      poolObjectData
    );

    return boundClientInstance;
  }

  public static findSignerPda(
    publicKey: PublicKey,
    memechanProgramId: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("signer"), publicKey.toBytes()],
      memechanProgramId
    )[0];
  }

  public static findBoundPoolPda(
    memeMintPubkey: PublicKey,
    solMintPubkey: PublicKey,
    memechanProgramId: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from("bound_pool"),
        memeMintPubkey.toBytes(),
        solMintPubkey.toBytes(),
      ],
      memechanProgramId
    )[0];
  }

  public static findStakingPda(
    memeMintPubkey: PublicKey,
    memechanProgramId: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("staking_pool"), memeMintPubkey.toBytes()],
      memechanProgramId
    )[0];
  }

  public static findMemeTicketPda(
    stakingPubKey: PublicKey,
    memechanProgramId: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("admin_ticket"), stakingPubKey.toBytes()],
      memechanProgramId
    )[0];
  }

  public static async getCreateNewBondingPoolAndTokenTransaction(
    args: GetCreateNewBondingPoolAndTokenTransactionArgs
  ) {
    const {
      admin,
      payer,
      client,
      quoteToken,
      transaction = new Transaction(),
      adminSolPublicKey,
      tokenMetadata,
      tokens_airdropped,
      vesting_linear_length,
    } = args;
    const { connection, memechanProgram } = client;

    const memeMintKeypair = Keypair.generate();
    const memeMint = memeMintKeypair.publicKey;
    const id = this.findBoundPoolPda(
      memeMintKeypair.publicKey,
      quoteToken.mint,
      args.client.memechanProgram.programId
    );
    const poolSigner = BoundPoolClient.findSignerPda(
      id,
      args.client.memechanProgram.programId
    );
    const createMemeMintWithPriorityInstructions = (
      await getCreateMintWithPriorityTransaction(
        connection,
        payer,
        poolSigner,
        null,
        MEMECHAN_MEME_TOKEN_DECIMALS,
        memeMintKeypair
      )
    ).instructions;
    transaction.add(...createMemeMintWithPriorityInstructions);
    let adminQuoteVault: PublicKey = await getAssociatedTokenAddress(
      QUOTE_MINT,
      BP_FEE_VAULT_OWNER,
      true
    );
    transaction.add(
      await createAssociatedTokenAccountIdempotentInstruction(
        payer,
        adminQuoteVault,
        BP_FEE_VAULT_OWNER,
        QUOTE_MINT,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      )
    );

    const poolQuoteVault = await getAssociatedTokenAddress(
      QUOTE_MINT,
      poolSigner,
      true
    );
    //const poolQuoteVault = poolQuoteVaultId.publicKey;
    // const createPoolQuoteVaultInstructions = await getCreateAccountInstructions(
    //   connection,
    //   payer,
    //   quoteToken.mint,
    //   poolSigner,
    //   poolQuoteVaultId
    // );
    const createPoolQuoteVaultInstructions =
      await getCreateAssociatedTokenAccountInstructions(
        poolQuoteVault,
        payer,
        QUOTE_MINT,
        poolSigner
      );
    transaction.add(...createPoolQuoteVaultInstructions);
    const launchVault = await getAssociatedTokenAddress(
      memeMint,
      poolSigner,
      true
    );
    // const launchVaultId = Keypair.generate();
    // const launchVault = launchVaultId.publicKey;
    // const createLaunchVaultInstructions =
    //   await getCreateTokenAccountInstructions(
    //     connection,
    //     payer,
    //     memeMint,
    //     poolSigner,
    //     launchVaultId
    //   );

    const createLaunchVaultInstructions =
      await getCreateAssociatedTokenAccountInstructions(
        launchVault,
        payer,
        memeMint,
        poolSigner
      );
    transaction.add(...createLaunchVaultInstructions);

    const createPoolInstruction = await memechanProgram.methods
      .newPool(new BN(tokens_airdropped), new BN(vesting_linear_length))
      .accounts({
        feeQuoteVault: adminQuoteVault,
        memeVault: launchVault,
        quoteVault: poolQuoteVault,
        memeMint: memeMint,
        pool: id,
        poolSigner: poolSigner,
        sender: payer,
        quoteMint: quoteToken.mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        targetConfig: TargetConfig.findTargetConfigPda(
          quoteToken.mint,
          memechan.programId
        ),
      })
      .instruction();

    transaction.add(createPoolInstruction);

    const createTokenInstructions = (
      await getCreateMetadataTransaction(client, {
        payer,
        mint: memeMint,
        poolSigner,
        poolId: id,
        metadata: tokenMetadata,
      })
    ).instructions;

    transaction.add(...createTokenInstructions);

    return {
      transaction,
      memeMintKeypair,
    };
  }

  public static async new(args: BoundPoolArgs): Promise<BoundPoolClient> {
    const { payer, client, quoteToken } = args;
    const { connection, memechanProgram } = client;

    const { transaction, memeMintKeypair } =
      await this.getCreateNewBondingPoolAndTokenTransaction({
        ...args,
        payer: payer.publicKey,
      });

    const memeMint = memeMintKeypair.publicKey;

    const size = getTxSize(transaction, payer.publicKey);
    console.debug("createPoolAndTokenSignature size: ", size);
    console.debug(payer.publicKey.toBase58());

    const lookupTableAccount = (
      await connection.getAddressLookupTable(args.lutAddr!)
    ).value;

    const blockhash = await client.connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash.blockhash;

    const txMessage = new TransactionMessage({
      instructions: transaction.instructions,
      payerKey: payer.publicKey,
      recentBlockhash: blockhash.blockhash,
    }).compileToV0Message([lookupTableAccount]);

    const transactionV0 = new VersionedTransaction(txMessage);

    transactionV0.sign([payer, memeMintKeypair]);
    transaction.sign(payer, memeMintKeypair);

    const luttxId = await client.connection.sendTransaction(transactionV0, {
      skipPreflight: true,
    });
    console.log(luttxId);

    // console.debug(
    //   "LUT createPoolAndTokenSignature size: ",
    //   transactionV0.serialize().length,
    //   "\nlegacy tx size: ",
    //   transaction.serialize().length
    // );

    // const createPoolAndTokenSignature = await provider.sendAndConfirm(
    //   transaction,
    //   [payer, memeMintKeypair, poolQuoteVaultId, launchVaultId],
    //   { skipPreflight: true }
    // );
    // console.log("createPoolAndTokenSignature:", createPoolAndTokenSignature);

    const id = this.findBoundPoolPda(
      memeMint,
      quoteToken.mint,
      memechanProgram.programId
    );
    await sleep(1000);
    const poolObjectData = await BoundPoolClient.fetch2(client.connection, id);

    return new BoundPoolClient(
      id,
      client,
      poolObjectData.memeReserve.vault,
      poolObjectData.quoteReserve.vault,
      memeMint,
      quoteToken.mint,
      new Token(TOKEN_PROGRAM_ID, memeMint, MEMECHAN_MEME_TOKEN_DECIMALS),
      poolObjectData
    );
  }

  public static async slowNew(args: BoundPoolArgs): Promise<BoundPoolClient> {
    const { admin, payer, client, quoteToken } = args;
    const { connection, memechanProgram } = client;

    const memeMintKeypair = Keypair.generate();
    const id = this.findBoundPoolPda(
      memeMintKeypair.publicKey,
      quoteToken.mint,
      args.client.memechanProgram.programId
    );
    const poolSigner = BoundPoolClient.findSignerPda(
      id,
      args.client.memechanProgram.programId
    );

    const memeMint = await createMintWithPriority(
      connection,
      payer,
      poolSigner,
      null,
      MEMECHAN_MEME_TOKEN_DECIMALS,
      memeMintKeypair,
      {
        skipPreflight: true,
        commitment: "confirmed",
      }
    );

    const adminSolVault = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        quoteToken.mint,
        admin,
        true,
        "confirmed",
        {
          skipPreflight: true,
        }
      )
    ).address;
    const poolQuoteVaultid = Keypair.generate();
    const poolQuoteVault = await createAccount(
      connection,
      payer,
      quoteToken.mint,
      poolSigner,
      poolQuoteVaultid,
      {
        skipPreflight: true,
        commitment: "confirmed",
      }
    );

    const launchVaultid = Keypair.generate();
    const launchVault = await createAccount(
      connection,
      payer,
      memeMint,
      poolSigner,
      launchVaultid,
      {
        skipPreflight: true,
        commitment: "confirmed",
      }
    );

    console.log(
      `pool id: ${id.toBase58()} memeMint: ${memeMint.toBase58()}, adminSolVault: ${adminSolVault.toBase58()}, poolQuoteVault: ${poolQuoteVault.toBase58()}, launchVault: ${launchVault.toBase58()}`
    );

    const newPoolTxDigest = await memechanProgram.methods
      .newPool()
      .accounts({
        feeQuoteVault: adminSolVault,
        memeVault: launchVault,
        quoteVault: poolQuoteVault,
        memeMint: memeMint,
        pool: id,
        poolSigner: poolSigner,
        sender: payer.publicKey,
        quoteMint: quoteToken.mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        targetConfig: TargetConfig.findTargetConfigPda(
          quoteToken.mint,
          memechan.programId
        ),
      })
      .signers([payer])
      .rpc({ skipPreflight: true });

    console.log("new pool tx result: " + newPoolTxDigest);

    await createMetadata(client, {
      payer,
      mint: memeMint,
      poolSigner,
      poolId: id,
      metadata: args.tokenMetadata,
    });

    // const coinApi = new CoinAPI();
    // const createCoinResponse = coinApi.({
    //   txDigest: newPoolTxDigest,
    // });

    // console.log("createCoinResponse: " + JSON.stringify(createCoinResponse));

    const poolObjectData = await BoundPoolClient.fetch2(client.connection, id);

    return new BoundPoolClient(
      id,
      client,
      launchVault,
      poolQuoteVault,
      memeMint,
      quoteToken.mint,
      new Token(TOKEN_PROGRAM_ID, memeMint, MEMECHAN_MEME_TOKEN_DECIMALS),
      poolObjectData
    );
  }

  /**
   * Fetches the bound pool account information.
   *
   * @deprecated Please use `fetch2` method
   * @param {Object} [program=this.client.memechanProgram] - The program to use for fetching the account.
   * @param {string} [accountId=this.id] - The ID of the account to fetch.
   * @returns {Promise<T>} - The account information.
   */
  async fetch(program = this.client.memechanProgram, accountId = this.id) {
    const accountInfo = await program.account.boundPool.fetch(
      accountId,
      "confirmed"
    );
    return accountInfo;
  }

  /**
   * Fetches the bound pool account information.
   *
   * @param {Connection} connection - The Solana RPC connection.
   * @param {PublicKey} accountId - The ID of the account to fetch.
   * @returns {Promise<T>} - The account information.
   */
  static async fetch2(connection: Connection, accountId: PublicKey) {
    const accountInfo = await memechan.account.boundPool.fetch(accountId);

    if (!accountInfo) {
      throw new Error(
        `[BoundPoolClient.fetch] No account info found for the pool ${accountId}`
      );
    }

    return accountInfo;
  }

  /**
   * Fetches the account information with retry logic.
   *
   * @param {Object} [program=this.client.memechanProgram] - The program to use for fetching the account.
   * @param {string} [accountId=this.id] - The ID of the account to fetch.
   * @param {number} [retries=3] - The number of retry attempts.
   * @param {number} [delay=1000] - The delay between retry attempts in milliseconds.
   * @returns {Promise<T>} - The account information.
   */
  async fetchWithRetry(
    program = this.client.memechanProgram,
    accountId = this.id,
    retries = 3,
    delay = 1000
  ) {
    return retry({
      fn: () => this.fetch(program, accountId),
      retries,
      delay,
      functionName: "fetch",
    });
  }

  public static async all(program: Program<MemechanSol>) {
    return program.account.boundPool.all();
  }

  public findSignerPda(): PublicKey {
    return BoundPoolClient.findSignerPda(
      this.id,
      this.client.memechanProgram.programId
    );
  }

  public static async airdropLiquidityTokens(
    mint: PublicKey,
    wallet: PublicKey,
    authority: Signer,
    provider: Provider,
    payer: Signer,
    amount: number = 1_000_000
  ) {
    return mintTo(provider.connection, payer, mint, wallet, authority, amount);
  }

  public async swapY(input: SwapYArgs): Promise<MemeTicket> {
    const user = input.user!;
    const payer = input.payer!;
    const ticketNumber = input.ticketNumber ?? 1;
    const pool = input.pool ?? this.id;
    const id = MemeTicket.getMemeTicketPDA({
      ticketNumber,
      poolId: pool,
      userId: user.publicKey,
    });
    const poolSignerPda = BoundPoolClient.findSignerPda(
      pool,
      this.client.memechanProgram.programId
    );
    const sol_in = input.quoteAmountIn;
    const meme_out = input.memeTokensOut;
    const referrerPoints = input.referrer;
    const userQuoteAcc =
      input.userQuoteAcc ??
      (
        await getOrCreateAssociatedTokenAccount(
          this.client.connection,
          payer,
          input.quoteMint,
          user.publicKey,
          true,
          "confirmed",
          { skipPreflight: true }
        )
      ).address;
    // const balance = await this.client.connection.getBalance(payer.publicKey);
    // console.log(`${balance / LAMPORTS_PER_SOL} SOL`);

    // const transferTx = new Transaction().add(
    //   //  modifyComputeUnits,
    //   // addPriorityFee,
    //   SystemProgram.transfer({
    //     fromPubkey: payer.publicKey,
    //     toPubkey: userSolAcc,
    //     lamports: BigInt(sol_in.toString()),
    //   }),
    //   createSyncNativeInstruction(userSolAcc),
    // );

    // const transferResult = await sendAndConfirmTransaction(this.client.connection, transferTx, [payer], {
    //   skipPreflight: true,
    //   commitment: "confirmed",
    // });

    //console.log("3 transferResult: " + transferResult);

    const userPoints = await getOrCreateAssociatedTokenAccount(
      this.client.connection,
      payer,
      pointsMint,
      user.publicKey,
      false,
      "processed",
      { skipPreflight: true }
    );
    const swapTxDig = await this.client.memechanProgram.methods
      .swapY(new BN(sol_in), new BN(meme_out), new BN(ticketNumber))
      .accounts({
        memeTicket: id,
        owner: user.publicKey,
        pool: pool,
        poolSignerPda: poolSignerPda,
        quoteVault: this.quoteVault,
        userSol: userQuoteAcc,
        pointsMint,
        pointsPda,
        pointsAcc,
        pointsEpoch: PointsEpochWrapper.pointsEpochPDA(),
        referrerPoints,
        userPoints: userPoints.address,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log(`swap tx ${swapTxDig}`);

    return new MemeTicket(id, this.client);
  }

  /**
   * Swaps a Y token (expecting `SLERF` token) for another asset by executing a buy meme transaction.
   * @param {SwapYArgs} input - The input arguments required for the swap.
   * @returns {Promise<string>} A promise that resolves to the transaction ID of the swap.
   * @throws {Error} Throws an error if the transaction creation or confirmation fails.
   * @untested This method is untested and may contain bugs.
   */
  public async buyMeme(input: BuyMemeArgs): Promise<string> {
    // TODO: Check whether user has enough amount of quoute token
    const { tx, memeTicketKeypair } = await this.getBuyMemeTransaction(input);

    const txId = await sendAndConfirmTransaction(
      this.client.connection,
      tx,
      [input.signer, memeTicketKeypair],
      {
        skipPreflight: true,
        commitment: "confirmed",
      }
    );

    return txId;
  }

  /**
   * Generates a transaction to buy a meme.
   *
   * @param {GetBuyMemeTransactionArgs} input - The input arguments required for the transaction.
   * @returns {Promise<GetBuyMemeTransactionOutput>} A promise that resolves to the transaction object.
   *
   * @work-in-progress This method is a work in progress and not yet ready for production use.
   * @untested This method is untested and may contain bugs.
   */
  public async getBuyMemeTransaction(
    input: GetBuyMemeTransactionArgs
  ): Promise<GetBuyMemeTransactionOutput> {
    const {
      inputAmount,
      minOutputAmount,
      slippagePercentage,
      user,
      ticketNumber,
      transaction = new Transaction(),
    } = input;
    let { inputTokenAccount } = input;

    const pool = this.id;
    const poolSignerPda = this.findSignerPda();
    const memeTicketKeypair = Keypair.generate();
    const connection = this.client.connection;

    // input
    const inputAmountWithDecimals = normalizeInputCoinAmount(
      inputAmount,
      MEMECHAN_QUOTE_TOKEN_DECIMALS
    );
    const inputAmountBN = new BN(inputAmountWithDecimals.toString());

    // output
    // Note: Be aware, we relay on the fact that `MEMECOIN_DECIMALS` would be always set same for all memecoins
    // As well as the fact that memecoins and tickets decimals are always the same
    const minOutputWithSlippage = deductSlippage(
      new BigNumber(minOutputAmount),
      slippagePercentage
    );
    const minOutputNormalized = normalizeInputCoinAmount(
      minOutputWithSlippage.toString(),
      MEMECHAN_MEME_TOKEN_DECIMALS
    );
    const minOutputBN = new BN(minOutputNormalized.toString());

    // If `inputTokenAccount` is not passed in args, we need to find out, whether a quote account for an admin
    // already exists
    if (!inputTokenAccount) {
      const associatedToken = getAssociatedTokenAddressSync(
        this.quoteTokenMint,
        user,
        true,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      const account = await getAccount(connection, associatedToken);
      inputTokenAccount = account.address;

      // If the quote account for the admin doesn't exist, add an instruction to create it
      if (!inputTokenAccount) {
        const associatedTransactionInstruction =
          createAssociatedTokenAccountInstruction(
            user,
            associatedToken,
            user,
            this.quoteTokenMint,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
          );

        transaction.add(associatedTransactionInstruction);

        inputTokenAccount = associatedToken;
      }
    }

    const buyMemeInstruction = await this.client.memechanProgram.methods
      .swapY(inputAmountBN, minOutputBN, new BN(ticketNumber))
      .accounts({
        memeTicket: memeTicketKeypair.publicKey,
        owner: user,
        pool: pool,
        poolSignerPda: poolSignerPda,
        quoteVault: this.quoteVault,
        userSol: inputTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    transaction.add(buyMemeInstruction);

    console.debug(
      "memeTicketPublicKey: ",
      memeTicketKeypair.publicKey.toString()
    );
    console.debug("inputTokenAccount: ", inputTokenAccount.toString());

    return { tx: transaction, memeTicketKeypair, inputTokenAccount };
  }

  public async getOutputAmountForBuyMeme(input: GetOutputAmountForBuyMeme) {
    const { tx, memeTicketKeypair } = await this.getBuyMemeTransaction({
      ...input,
      minOutputAmount: "0",
    });

    const result = await this.client.connection.simulateTransaction(
      tx,
      [input.signer, memeTicketKeypair],
      true
    );

    // If error happened (e.g. pool is locked)
    if (result.value.err) {
      return {
        outputAmount: 0,
        error: result.value.err,
        logs: result.value.logs,
      };
    }

    // TODO: Decode the result of swap simulation

    return result;
  }

  public async isMemeCoinReadyToLivePhase() {
    const poolData = await BoundPoolClient.fetch2(
      this.client.connection,
      this.id
    );
    const isPoolLocked = poolData.locked;

    return isPoolLocked;
  }

  // TODO: Add method for checking is pool locked or not

  public async swapX(input: SwapXArgs): Promise<string> {
    const sellMemeCoinTransaction = await this.getSellMemeTransaction(input);

    const txId = await sendAndConfirmTransaction(
      this.client.connection,
      sellMemeCoinTransaction,
      [input.user],
      {
        skipPreflight: true,
        commitment: "confirmed",
      }
    );

    return txId;
  }

  public async getInitStakingPoolTransaction(
    input: GetInitStakingPoolTransactionArgs
  ): Promise<{
    transaction: Transaction;
    staking: PublicKey;
    stakingQuoteVault: PublicKey;
    stakingMemeVault: PublicKey;
  }> {
    const { user, payer, pool = this.id, boundPoolInfo } = input;
    const tx = input.transaction ?? new Transaction();

    const stakingId = BoundPoolClient.findStakingPda(
      boundPoolInfo.memeReserve.mint,
      this.client.memechanProgram.programId
    );
    const stakingSigner = StakingPool.findSignerPda(
      stakingId,
      this.client.memechanProgram.programId
    );
    const adminTicketId = BoundPoolClient.findMemeTicketPda(
      stakingId,
      this.client.memechanProgram.programId
    );
    const stakingQuoteVault = await ensureAssociatedTokenAccountWithIX({
      connection: this.client.connection,
      payer: payer.publicKey,
      mint: boundPoolInfo.quoteReserve.mint,
      owner: stakingSigner,
      transaction: tx,
    });

    const stakingMemeVault = await ensureAssociatedTokenAccountWithIX({
      connection: this.client.connection,
      payer: payer.publicKey,
      mint: boundPoolInfo.memeReserve.mint,
      owner: stakingSigner,
      transaction: tx,
    });

    const stakingChanVault = await ensureAssociatedTokenAccountWithIX({
      connection: this.client.connection,
      payer: payer.publicKey,
      mint: new PublicKey(CHAN_TOKEN_INFO.address),
      owner: stakingSigner,
      transaction: tx,
    });

    const airdropTokenVault = (
      await getOrCreateAssociatedTokenAccount(
        this.client.connection,
        payer,
        this.memeTokenMint,
        ADMIN_PUB_KEY,
        false
      )
    ).address;

    const initStakingPoolInstruction = await memechan.methods
      .initStakingPool()
      .accounts({
        pool,
        signer: user,
        boundPoolSignerPda: this.findSignerPda(),
        memeTicket: adminTicketId,
        poolMemeVault: boundPoolInfo.memeReserve.vault,
        poolQuoteVault: boundPoolInfo.quoteReserve.vault,
        stakingMemeVault,
        stakingQuoteVault: stakingQuoteVault,
        stakingChanVault: stakingChanVault,
        memeMint: boundPoolInfo.memeReserve.mint,
        quoteMint: this.quoteTokenMint,
        staking: stakingId,
        stakingPoolSignerPda: stakingSigner,
        feeVaultQuote: boundPoolInfo.feeVaultQuote,

        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    tx.add(initStakingPoolInstruction);

    return {
      transaction: tx,
      staking: stakingId,
      stakingMemeVault,
      stakingQuoteVault,
    };
  }

  public async initStakingPool(
    input: InitStakingPoolArgs
  ): Promise<InitStakingPoolResult> {
    const { transaction, staking, stakingMemeVault, stakingQuoteVault } =
      await this.getInitStakingPoolTransaction({
        ...input,
        user: input.user.publicKey,
      });

    const signAndConfirmInitStakingPoolTransaction =
      getSendAndConfirmTransactionMethod({
        connection: this.client.connection,
        transaction,
        signers: [input.user],
      });

    await retry({
      fn: signAndConfirmInitStakingPoolTransaction,
      functionName: "initStakingPool",
    });

    return { staking, stakingMemeVault, stakingQuoteVault };
  }

  public async getSellMemeTransaction(
    input: GetSellMemeTransactionArgs
  ): Promise<Transaction> {
    const tx = input.transaction ?? new Transaction();
    const user = input.user;

    const pool = this.id;
    const poolSignerPda = this.findSignerPda();
    const meme_in = input.memeAmountIn;
    const minQuoteAmountOut = input.minQuoteAmountOut;

    const memeTicket = input.userMemeTicket;
    const userSolAcc = input.userQuoteAcc;

    const sellMemeTransactionInstruction =
      await this.client.memechanProgram.methods
        .swapX(new BN(meme_in), new BN(minQuoteAmountOut))
        .accounts({
          pool: pool,
          memeTicket: memeTicket.id,
          userSol: userSolAcc,
          quoteVault: this.quoteVault,
          owner: user.publicKey,
          poolSigner: poolSignerPda,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .instruction();

    tx.add(sellMemeTransactionInstruction);

    return tx;
  }

  public async getInitQuoteAmmPoolTransaction(
    args: GetGoLiveTransactionArgs
  ): Promise<{
    goLiveTransaction: VersionedTransaction;
    stakingId: PublicKey;
  }> {
    const {
      boundPoolInfo,
      user,
      feeDestinationWalletAddress,
      memeVault,
      quoteVault,
      transaction = new Transaction(),
      tokenInfoA,
      tokenInfoB,
    } = args;
    const stakingId = BoundPoolClient.findStakingPda(
      boundPoolInfo.memeReserve.mint,
      this.client.memechanProgram.programId
    );
    const stakingSigner = StakingPool.findSignerPda(
      stakingId,
      this.client.memechanProgram.programId
    );

    transaction.add(
      SystemProgram.transfer({
        fromPubkey: user.publicKey,
        toPubkey: stakingSigner,
        lamports: 2_000_000_000,
      })
    );

    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
      units: 500000,
    });

    transaction.add(modifyComputeUnits);

    const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 5000000,
    });

    transaction.add(addPriorityFee);

    const tradeFeeBps = new BN(100);

    const { vaultProgram, ammProgram } = createProgram(provider.connection);

    const tokenAMint = new PublicKey(tokenInfoA.address);
    const tokenBMint = new PublicKey(tokenInfoB.address);
    const [
      { vaultPda: aVault, tokenVaultPda: aTokenVault, lpMintPda: aLpMintPda },
      { vaultPda: bVault, tokenVaultPda: bTokenVault, lpMintPda: bLpMintPda },
    ] = [
      getVaultPdas(tokenAMint, vaultProgram.programId),
      getVaultPdas(tokenBMint, vaultProgram.programId),
    ];
    const [aVaultAccount, bVaultAccount] = await Promise.all([
      vaultProgram.account.vault.fetchNullable(aVault),
      vaultProgram.account.vault.fetchNullable(bVault),
    ]);

    let aVaultLpMint = aLpMintPda;
    let bVaultLpMint = bLpMintPda;
    let preInstructions: Array<solana.TransactionInstruction> = [];

    if (!aVaultAccount) {
      const createVaultAIx =
        await VaultImpl.createPermissionlessVaultInstruction(
          provider.connection,
          user.publicKey,
          tokenInfoA
        );
      createVaultAIx && preInstructions.push(createVaultAIx);
    } else {
      aVaultLpMint = aVaultAccount.lpMint; // Old vault doesn't have lp mint pda
    }
    if (!bVaultAccount) {
      const createVaultBIx =
        await VaultImpl.createPermissionlessVaultInstruction(
          provider.connection,
          user.publicKey,
          tokenInfoB
        );
      createVaultBIx && preInstructions.push(createVaultBIx);
    } else {
      bVaultLpMint = bVaultAccount.lpMint; // Old vault doesn't have lp mint pda
    }

    console.log(
      await sendAndConfirmTransaction(
        provider.connection,
        new Transaction().add(...preInstructions),
        [user]
      )
    );

    const poolPubkey = derivePoolAddress(
      provider.connection,
      tokenInfoA,
      tokenInfoB,
      false,
      tradeFeeBps
    );
    const [[aVaultLp], [bVaultLp]] = [
      PublicKey.findProgramAddressSync(
        [aVault.toBuffer(), poolPubkey.toBuffer()],
        ammProgram.programId
      ),
      PublicKey.findProgramAddressSync(
        [bVault.toBuffer(), poolPubkey.toBuffer()],
        ammProgram.programId
      ),
    ];

    const [[adminTokenAFee], [adminTokenBFee]] = [
      PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.FEE), tokenAMint.toBuffer(), poolPubkey.toBuffer()],
        ammProgram.programId
      ),
      PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.FEE), tokenBMint.toBuffer(), poolPubkey.toBuffer()],
        ammProgram.programId
      ),
    ];

    const [lpMint] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.LP_MINT), poolPubkey.toBuffer()],
      ammProgram.programId
    );

    const [mintMetadata, _mintMetadataBump] = deriveMintMetadata(lpMint);

    const [lockEscrowPK] = deriveLockEscrowPda(
      poolPubkey,
      stakingSigner,
      ammProgram.programId
    );

    preInstructions = [];

    const payerPoolLp = await getAssociatedTokenAccount(lpMint, stakingSigner);

    const escrowAta = await getAssociatedTokenAccount(lpMint, lockEscrowPK);

    const goLiveInstruction = await this.client.memechanProgram.methods
      .initMemeAmmPool()
      .accounts({
        adminTokenAFee,
        adminTokenBFee,
        ammPool: poolPubkey,
        aTokenVault,
        aVault,
        aVaultLp,
        aVaultLpMint,
        bTokenVault,
        bVault,
        bVaultLp,
        bVaultLpMint,
        lpMint,
        mintMetadata,
        escrowVault: escrowAta,
        feeOwner: FEE_OWNER,
        lockEscrow: lockEscrowPK,
        payerPoolLp: payerPoolLp,
        quoteMint: tokenBMint,
        memeMint: boundPoolInfo.memeReserve.mint,

        staking: stakingId,
        stakingMemeVault: memeVault,
        stakingPoolSignerPda: stakingSigner,
        stakingQuoteVault: quoteVault,
        signer: user.publicKey,

        rent: SYSVAR_RENT_PUBKEY,
        ataProgram: ATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        metadataProgram: METADATA_PROGRAM_ID,
        ammProgram: ammProgram.programId,
        vaultProgram: vaultProgram.programId,
      })
      .instruction();

    transaction.add(goLiveInstruction);

    const [createLUTix, LUTaddr] =
      solana.AddressLookupTableProgram.createLookupTable({
        authority: admin,
        payer: admin,
        recentSlot: LUT2SLOT,
      });
    const extendIxs = solana.AddressLookupTableProgram.extendLookupTable({
      payer: admin,
      lookupTable: LUTaddr,
      authority: admin,
      addresses: [
        admin,
        SystemProgram.programId,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_PROGRAM_ID,
        memechan.programId,
        TargetConfig.findTargetConfigPda(QUOTE_MINT, memechan.programId),
        poolPubkey,
        user.publicKey,
        FEE_OWNER,
        SYSVAR_RENT_PUBKEY,
        QUOTE_MINT,
      ],
    });

    const tx = new Transaction().add(createLUTix, extendIxs);
    const txDig = await sendAndConfirmTransaction(provider.connection, tx, [
      adminSigner,
    ]);

    await sleep(1000);

    const lutAddr = getLUTPDA({
      authority: admin,
      recentSlot: LUT2SLOT,
    });

    const lookupTableAccount = (
      await provider.connection.getAddressLookupTable(lutAddr)
    ).value;

    const blockhash = await provider.connection.getLatestBlockhash();

    const txMessage = new TransactionMessage({
      instructions: transaction.instructions,
      payerKey: user.publicKey,
      recentBlockhash: blockhash.blockhash,
    }).compileToV0Message([lookupTableAccount]);

    const transactionV0 = new VersionedTransaction(txMessage);

    transactionV0.sign([user]);

    return {
      goLiveTransaction: transactionV0,
      stakingId,
    };
  }

  public async getInitChanAmmPoolTransaction(
    args: GetInitChanPoolTransactionArgs
  ): Promise<{
    goLiveTransaction: VersionedTransaction;
    stakingId: PublicKey;
  }> {
    const {
      boundPoolInfo,
      user,
      feeDestinationWalletAddress,
      memeVault,
      transaction = new Transaction(),
      tokenInfoA,
      tokenInfoB,
      chanSwap,
    } = args;
    const stakingId = BoundPoolClient.findStakingPda(
      boundPoolInfo.memeReserve.mint,
      this.client.memechanProgram.programId
    );
    const stakingSigner = StakingPool.findSignerPda(
      stakingId,
      this.client.memechanProgram.programId
    );

    transaction.add(
      SystemProgram.transfer({
        fromPubkey: user.publicKey,
        toPubkey: stakingSigner,
        lamports: 2_000_000_000,
      })
    );

    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
      units: 550000,
    });

    transaction.add(modifyComputeUnits);

    const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 5500000,
    });

    transaction.add(addPriorityFee);

    const tradeFeeBps = new BN(100);

    const { vaultProgram, ammProgram } = createProgram(provider.connection);

    const tokenAMint = new PublicKey(tokenInfoA.address);
    const tokenBMint = new PublicKey(tokenInfoB.address);
    const [
      { vaultPda: aVault, tokenVaultPda: aTokenVault, lpMintPda: aLpMintPda },
      { vaultPda: bVault, tokenVaultPda: bTokenVault, lpMintPda: bLpMintPda },
    ] = [
      getVaultPdas(tokenAMint, vaultProgram.programId),
      getVaultPdas(tokenBMint, vaultProgram.programId),
    ];
    const [aVaultAccount, bVaultAccount] = await Promise.all([
      vaultProgram.account.vault.fetchNullable(aVault),
      vaultProgram.account.vault.fetchNullable(bVault),
    ]);

    let aVaultLpMint = aLpMintPda;
    let bVaultLpMint = bLpMintPda;
    let preInstructions: Array<solana.TransactionInstruction> = [];

    if (!aVaultAccount) {
      const createVaultAIx =
        await VaultImpl.createPermissionlessVaultInstruction(
          provider.connection,
          user.publicKey,
          tokenInfoA
        );
      createVaultAIx && preInstructions.push(createVaultAIx);
    } else {
      aVaultLpMint = aVaultAccount.lpMint; // Old vault doesn't have lp mint pda
    }
    if (!bVaultAccount) {
      const createVaultBIx =
        await VaultImpl.createPermissionlessVaultInstruction(
          provider.connection,
          user.publicKey,
          tokenInfoB
        );
      createVaultBIx && preInstructions.push(createVaultBIx);
    } else {
      bVaultLpMint = bVaultAccount.lpMint; // Old vault doesn't have lp mint pda
    }
    try {
      console.log(
        await sendAndConfirmTransaction(
          provider.connection,
          new Transaction().add(...preInstructions),
          [user]
        )
      );
    } catch (e) {}

    const poolPubkey = derivePoolAddress(
      provider.connection,
      tokenInfoA,
      tokenInfoB,
      false,
      tradeFeeBps
    );
    const [[aVaultLp], [bVaultLp]] = [
      PublicKey.findProgramAddressSync(
        [aVault.toBuffer(), poolPubkey.toBuffer()],
        ammProgram.programId
      ),
      PublicKey.findProgramAddressSync(
        [bVault.toBuffer(), poolPubkey.toBuffer()],
        ammProgram.programId
      ),
    ];

    const [[adminTokenAFee], [adminTokenBFee]] = [
      PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.FEE), tokenAMint.toBuffer(), poolPubkey.toBuffer()],
        ammProgram.programId
      ),
      PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.FEE), tokenBMint.toBuffer(), poolPubkey.toBuffer()],
        ammProgram.programId
      ),
    ];

    const [lpMint] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.LP_MINT), poolPubkey.toBuffer()],
      ammProgram.programId
    );

    const [mintMetadata, _mintMetadataBump] = deriveMintMetadata(lpMint);

    const [lockEscrowPK] = deriveLockEscrowPda(
      poolPubkey,
      stakingSigner,
      ammProgram.programId
    );

    preInstructions = [];

    const payerPoolLp = await getAssociatedTokenAccount(lpMint, stakingSigner);

    const escrowAta = await getAssociatedTokenAccount(lpMint, lockEscrowPK);

    const fetchedChanSwap =
      await this.client.memechanProgram.account.chanSwap.fetch(chanSwap);

    const staking = await this.client.memechanProgram.account.stakingPool.fetch(
      stakingId
    );

    const swapFeeVault = await getAssociatedTokenAccount(
      QUOTE_MINT,
      SWAP_FEE_VAULT_OWNER
    );

    const goLiveInstruction = await this.client.memechanProgram.methods
      .initChanAmmPool()
      .accounts({
        adminTokenAFee,
        adminTokenBFee,
        ammPool: poolPubkey,
        aTokenVault,
        aVault,
        aVaultLp,
        aVaultLpMint,
        bTokenVault,
        bVault,
        bVaultLp,
        bVaultLpMint,
        lpMint,
        mintMetadata,
        escrowVault: escrowAta,
        feeOwner: FEE_OWNER,
        lockEscrow: lockEscrowPK,
        payerPoolLp: payerPoolLp,
        chanMint: tokenBMint,
        memeMint: boundPoolInfo.memeReserve.mint,

        staking: stakingId,
        stakingMemeVault: memeVault,
        stakingPoolSignerPda: stakingSigner,
        stakingChanVault: staking.chanVault,
        stakingQuoteVault: staking.quoteVault,

        feeQuoteVault: swapFeeVault,
        chanSwap,
        chanSwapSignerPda: ChanSwapWrapper.chanSwapSigner(),
        chanSwapVault: fetchedChanSwap.chanVault,

        signer: user.publicKey,

        rent: SYSVAR_RENT_PUBKEY,
        ataProgram: ATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        metadataProgram: METADATA_PROGRAM_ID,
        ammProgram: ammProgram.programId,
        vaultProgram: vaultProgram.programId,
      })
      .instruction();

    transaction.add(
      await createAssociatedTokenAccountIdempotentInstruction(
        user.publicKey,
        swapFeeVault,
        SWAP_FEE_VAULT_OWNER,
        QUOTE_MINT,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      )
    );

    transaction.add(goLiveInstruction);

    const [createLUTix, LUTaddr] =
      solana.AddressLookupTableProgram.createLookupTable({
        authority: admin,
        payer: admin,
        recentSlot: LUT2SLOT,
      });
    const extendIxs = solana.AddressLookupTableProgram.extendLookupTable({
      payer: admin,
      lookupTable: LUTaddr,
      authority: admin,
      addresses: [
        admin,
        SystemProgram.programId,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_PROGRAM_ID,
        memechan.programId,
        TargetConfig.findTargetConfigPda(QUOTE_MINT, memechan.programId),
        poolPubkey,
        user.publicKey,
        FEE_OWNER,
        swapFeeVault,
        SYSVAR_RENT_PUBKEY,
        memeVault,
        stakingId,
        QUOTE_MINT,
        stakingSigner,
        new PublicKey(CHAN_TOKEN_INFO.address),
        ChanSwapWrapper.chanSwapId(),
        ChanSwapWrapper.chanSwapSigner(),
      ],
    });

    const tx = new Transaction().add(createLUTix, extendIxs);
    const txDig = await sendAndConfirmTransaction(provider.connection, tx, [
      adminSigner,
    ]);

    await sleep(1000);

    const lutAddr = getLUTPDA({
      authority: admin,
      recentSlot: LUT2SLOT,
    });

    const lookupTableAccount = (
      await provider.connection.getAddressLookupTable(lutAddr)
    ).value;

    const blockhash = await provider.connection.getLatestBlockhash();

    const txMessage = new TransactionMessage({
      instructions: transaction.instructions,
      payerKey: user.publicKey,
      recentBlockhash: blockhash.blockhash,
    }).compileToV0Message([lookupTableAccount]);

    const transactionV0 = new VersionedTransaction(txMessage);

    transactionV0.sign([user]);

    return {
      goLiveTransaction: transactionV0,
      stakingId,
    };
  }

  public async initQuoteAmmPool(args: GoLiveArgs): Promise<StakingPool> {
    console.debug("initQuoteAmmPool Begin");
    // Get needed transactions
    const { goLiveTransaction, stakingId } =
      await this.getInitQuoteAmmPoolTransaction(args);

    // Send transaction to go live
    const goLiveSignature = await provider.connection.sendTransaction(
      goLiveTransaction,
      { skipPreflight: true }
    );

    console.debug("go live signature:", goLiveSignature);

    // Check go live succeeded
    const {
      blockhash: blockhash1,
      lastValidBlockHeight: lastValidBlockHeight1,
    } = await this.client.connection.getLatestBlockhash("confirmed");
    const goLiveTxResult = await this.client.connection.confirmTransaction(
      {
        signature: goLiveSignature,
        blockhash: blockhash1,
        lastValidBlockHeight: lastValidBlockHeight1,
      },
      "confirmed"
    );

    if (goLiveTxResult.value.err) {
      console.error("goLiveTxResult:", goLiveTxResult);
      throw new Error("goLiveTxResult failed");
    }

    const stakingPoolInstance = await StakingPool.fromStakingPoolId({
      client: this.client,
      poolAccountAddressId: stakingId,
    });

    return stakingPoolInstance;
  }

  public async initChanAmmPool(args: InitChanAmmPool) {
    console.debug("initChanAmmPool Begin");
    // Get needed transactions

    const { goLiveTransaction, stakingId } =
      await this.getInitChanAmmPoolTransaction(args);

    // Send transaction to go live
    const goLiveSignature = await provider.connection.sendTransaction(
      goLiveTransaction,
      { skipPreflight: true }
    );

    console.debug("initChanAmmPool signature:", goLiveSignature);

    // Check go live succeeded
    const {
      blockhash: blockhash1,
      lastValidBlockHeight: lastValidBlockHeight1,
    } = await this.client.connection.getLatestBlockhash("confirmed");
    const goLiveTxResult = await this.client.connection.confirmTransaction(
      {
        signature: goLiveSignature,
        blockhash: blockhash1,
        lastValidBlockHeight: lastValidBlockHeight1,
      },
      "confirmed"
    );

    if (goLiveTxResult.value.err) {
      console.error("goLiveTxResult:", goLiveTxResult);
      throw new Error("goLiveTxResult failed");
    }

    const stakingPoolInstance = await StakingPool.fromStakingPoolId({
      client: this.client,
      poolAccountAddressId: stakingId,
    });

    return stakingPoolInstance;
  }

  public async fetchRelatedTickets() {
    return MemeTicket.fetchRelatedTickets(this.id, this.client);
  }

  public async getHoldersCount() {
    return BoundPoolClient.getHoldersCount(this.id, this.client);
  }

  public async getHoldersMap() {
    return BoundPoolClient.getHoldersMap(this.id, this.client);
  }

  public async getHoldersList() {
    return BoundPoolClient.getHoldersList(this.id, this.client);
  }

  /**
   * Fetches all unique token holders for pool and returns their number
   */
  public static async getHoldersCount(pool: PublicKey, client: MemechanClient) {
    return (await BoundPoolClient.getHoldersList(pool, client)).length;
  }

  public static async getHoldersMap(pool: PublicKey, client: MemechanClient) {
    const tickets = await MemeTicket.fetchRelatedTickets(pool, client);
    const uniqueHolders: Map<string, MemeTicketFields[]> = new Map();

    tickets.forEach((ticket) => {
      const addr = ticket.owner.toBase58();
      if (!uniqueHolders.has(addr)) {
        uniqueHolders.set(addr, []);
      }
      uniqueHolders.get(addr)?.push(ticket);
    });

    return uniqueHolders;
  }

  /**
   * Fetches all unique token holders for pool and returns thier addresses
   */
  public static async getHoldersList(pool: PublicKey, client: MemechanClient) {
    const holdersMap = await BoundPoolClient.getHoldersMap(pool, client);

    return Array.from(holdersMap.keys());
  }

  public static async getMemePrice({
    boundPoolInfo,
    quotePriceInUsd,
  }: {
    boundPoolInfo: BoundPoolType;
    quotePriceInUsd: number;
  }): Promise<{ priceInQuote: string; priceInUsd: string }> {
    const memeBalance = new BigNumber(
      boundPoolInfo.memeReserve.tokens.toString()
    );
    const quoteBalance = new BigNumber(
      boundPoolInfo.quoteReserve.tokens.toString()
    );

    const quoteBalanceConverted = quoteBalance.div(
      10 ** MEMECHAN_QUOTE_TOKEN_DECIMALS
    );
    const soldMemeConverted = new BigNumber(DEFAULT_MAX_M)
      .minus(memeBalance)
      .div(10 ** MEMECHAN_MEME_TOKEN_DECIMALS);

    // In case no meme coins were sold, return 0-prices
    if (soldMemeConverted.eq(0)) {
      return { priceInQuote: "0", priceInUsd: "0" };
    }

    const memePriceInQuote = quoteBalanceConverted.div(soldMemeConverted);
    const memePriceInUsd = memePriceInQuote
      .multipliedBy(quotePriceInUsd)
      .toString();

    return {
      priceInQuote: memePriceInQuote.toString(),
      priceInUsd: memePriceInUsd,
    };
  }

  public static getMemeMarketCap({
    memePriceInUsd,
  }: {
    memePriceInUsd: string;
  }): string {
    const fullMemeAmountConverted = new BigNumber(DEFAULT_MAX_M_LP)
      .plus(DEFAULT_MAX_M)
      .div(10 ** MEMECHAN_MEME_TOKEN_DECIMALS);

    const marketCap = fullMemeAmountConverted
      .multipliedBy(memePriceInUsd)
      .toString();

    return marketCap;
  }

  static getATAAddress(
    owner: PublicKey,
    mint: PublicKey,
    programId: PublicKey
  ) {
    return findProgramAddress(
      [owner.toBuffer(), programId.toBuffer(), mint.toBuffer()],
      new PublicKey(ATA_PROGRAM_ID)
    );
  }

  static getAssociatedId({
    programId,
    configId,
    mint0,
    mint1,
  }: {
    programId: PublicKey;
    configId: PublicKey;
    mint0: PublicKey;
    mint1: PublicKey;
  }) {
    const { publicKey } = findProgramAddress(
      [
        Buffer.from("amm_associated_seed", "utf-8"),
        configId.toBuffer(),
        mint0.toBuffer(),
        mint1.toBuffer(),
      ],
      programId
    );
    return publicKey;
  }

  static getAssociatedAuthority({ programId }: { programId: PublicKey }) {
    return findProgramAddress(
      // new Uint8Array(Buffer.from('amm authority'.replace('\u00A0', ' '), 'utf-8'))
      [
        Buffer.from([
          97, 109, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121,
        ]),
      ],
      programId
    );
  }

  static getAssociatedBaseVault({
    programId,
    marketId,
  }: {
    programId: PublicKey;
    marketId: PublicKey;
  }) {
    const { publicKey } = findProgramAddress(
      [
        programId.toBuffer(),
        marketId.toBuffer(),
        Buffer.from("coin_vault_associated_seed", "utf-8"),
      ],
      programId
    );
    return publicKey;
  }

  static getAssociatedQuoteVault({
    programId,
    marketId,
  }: {
    programId: PublicKey;
    marketId: PublicKey;
  }) {
    const { publicKey } = findProgramAddress(
      [
        programId.toBuffer(),
        marketId.toBuffer(),
        Buffer.from("pc_vault_associated_seed", "utf-8"),
      ],
      programId
    );
    return publicKey;
  }

  static getAssociatedLpMint({
    programId,
    marketId,
  }: {
    programId: PublicKey;
    marketId: PublicKey;
  }) {
    const { publicKey } = findProgramAddress(
      [
        programId.toBuffer(),
        marketId.toBuffer(),
        Buffer.from("lp_mint_associated_seed", "utf-8"),
      ],
      programId
    );
    return publicKey;
  }

  static getAssociatedLpVault({
    programId,
    marketId,
  }: {
    programId: PublicKey;
    marketId: PublicKey;
  }) {
    const { publicKey } = findProgramAddress(
      [
        programId.toBuffer(),
        marketId.toBuffer(),
        Buffer.from("temp_lp_token_associated_seed", "utf-8"),
      ],
      programId
    );
    return publicKey;
  }

  static getAssociatedTargetOrders({
    programId,
    marketId,
  }: {
    programId: PublicKey;
    marketId: PublicKey;
  }) {
    const { publicKey } = findProgramAddress(
      [
        programId.toBuffer(),
        marketId.toBuffer(),
        Buffer.from("target_associated_seed", "utf-8"),
      ],
      programId
    );
    return publicKey;
  }

  static getAssociatedWithdrawQueue({
    programId,
    marketId,
  }: {
    programId: PublicKey;
    marketId: PublicKey;
  }) {
    const { publicKey } = findProgramAddress(
      [
        programId.toBuffer(),
        marketId.toBuffer(),
        Buffer.from("withdraw_associated_seed", "utf-8"),
      ],
      programId
    );
    return publicKey;
  }

  static getAssociatedOpenOrders({
    programId,
    marketId,
  }: {
    programId: PublicKey;
    marketId: PublicKey;
  }) {
    const { publicKey } = findProgramAddress(
      [
        programId.toBuffer(),
        marketId.toBuffer(),
        Buffer.from("open_order_associated_seed", "utf-8"),
      ],
      programId
    );
    return publicKey;
  }

  static getAssociatedConfigId({ programId }: { programId: PublicKey }) {
    const { publicKey } = findProgramAddress(
      [Buffer.from("amm_config_account_seed", "utf-8")],
      programId
    );
    return new PublicKey("9zSzfkYy6awexsHvmggeH36pfVUdDGyCcwmjT3AQPBj6");
    return publicKey;
  }

  static getObservationAddr({
    programId,
    poolState,
  }: {
    programId: PublicKey;
    poolState: PublicKey;
  }) {
    const { publicKey } = findProgramAddress(
      [Buffer.from("observation", "utf-8"), poolState.toBuffer()],
      programId
    );
    return publicKey;
  }

  async airdrop(
    connection: Connection,
    to: PublicKey,
    amount: number = 5_000_000_000
  ) {
    await connection.confirmTransaction(
      await connection.requestAirdrop(to, amount),
      "confirmed"
    );
  }
}

async function getUserLockEscrow(owner: PublicKey): Promise<LockEscrow | null> {
  const [lockEscrow, _lockEscrowBump] = deriveLockEscrowPda(
    this.address,
    owner,
    this.program.programId
  );
  const lockEscrowAccount: LockEscrowAccount | null =
    await this.program.account.lockEscrow.fetchNullable(lockEscrow);
  if (!lockEscrowAccount) return null;
  const unClaimedFee = calculateUnclaimedLockEscrowFee(
    lockEscrowAccount.totalLockedAmount,
    lockEscrowAccount.lpPerToken,
    lockEscrowAccount.unclaimedFeePending,
    this.poolInfo.virtualPriceRaw
  );

  const { tokenAOutAmount, tokenBOutAmount } = this.getWithdrawQuote(
    unClaimedFee,
    0
  );
  return {
    address: lockEscrow,
    amount: lockEscrowAccount.totalLockedAmount || new BN(0),
    fee: {
      claimed: {
        tokenA: lockEscrowAccount.aFee || new BN(0),
        tokenB: lockEscrowAccount.bFee || new BN(0),
      },
      unClaimed: {
        lp: unClaimedFee,
        tokenA: tokenAOutAmount || new BN(0),
        tokenB: tokenBOutAmount || new BN(0),
      },
    },
  };
}
