import { ApiPoolInfoV4, Token } from "@raydium-io/raydium-sdk";
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
import { ATA_PROGRAM_ID, PROGRAMIDS } from "../raydium/config";
import {
  createMarket,
  getCreateMarketTransactions,
} from "../raydium/openBookCreateMarket";
import { StakingPool } from "../staking-pool/StakingPool";
import {
  BoundPoolArgs,
  BuyMemeArgs,
  GetBuyMemeTransactionArgs,
  GetBuyMemeTransactionOutput,
  GetCreateNewBondingPoolAndTokenTransactionArgs,
  GetGoLiveTransactionArgs,
  GetInitStakingPoolTransactionArgs,
  GetOutputAmountForBuyMeme,
  GetSellMemeTransactionArgs,
  GoLiveArgs,
  InitStakingPoolArgs,
  InitStakingPoolResult,
  SwapXArgs,
  SwapYArgs,
} from "./types";

import { findProgramAddress } from "../common/helpers";
import {
  DEFAULT_MAX_M,
  DEFAULT_MAX_M_LP,
  MEMECHAN_MEME_TOKEN_DECIMALS,
  MEMECHAN_QUOTE_MINT,
  MEMECHAN_QUOTE_TOKEN,
  MEMECHAN_QUOTE_TOKEN_DECIMALS,
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
import { QUOTE_MINT, admin, memechan, provider, sleep } from "../../helpers";
import { MemechanSol } from "../../../target/types/memechan_sol";
import { BoundPoolType } from "../../bound_pool";
import { CPMM, FEE_VAULT } from "../../common";
import {
  getAuthAddress,
  getOrcleAccountAddress,
  getPoolAddress,
  getPoolLpMintAddress,
  getPoolVaultAddress,
} from "../../raydium/utils/pda";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

export class BoundPoolClient {
  private constructor(
    public id: PublicKey,
    public client: MemechanClient,
    public memeVault: PublicKey,
    public quoteVault: PublicKey,
    public memeTokenMint: PublicKey,
    public quoteTokenMint: PublicKey = MEMECHAN_QUOTE_MINT,
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
    } = args;
    const { connection, memechanProgram } = client;
    console.log(payer, quoteToken, adminSolPublicKey);

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
    console.log(memeMint);

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
    console.log(payer, poolSigner, memeMintKeypair.publicKey);
    transaction.add(...createMemeMintWithPriorityInstructions);

    let adminQuoteVault: PublicKey = FEE_VAULT;

    console.log(adminQuoteVault);

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
      .newPool()
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

    console.log(connection.rpcEndpoint.toString());

    console.log("trying LUT");
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

    const luttxId = await client.connection.sendTransaction(transactionV0);
    console.log(luttxId);

    console.debug(
      "LUT createPoolAndTokenSignature size: ",
      transactionV0.serialize().length,
      "\nlegacy tx size: ",
      transaction.serialize().length
    );

    // const createPoolAndTokenSignature = await provider.sendAndConfirm(
    //   transaction,
    //   [payer, memeMintKeypair, poolQuoteVaultId, launchVaultId],
    //   { skipPreflight: true }
    // );
    // console.log("createPoolAndTokenSignature:", createPoolAndTokenSignature);

    console.log("pid", memechanProgram.programId);
    const id = this.findBoundPoolPda(
      memeMint,
      quoteToken.mint,
      memechanProgram.programId
    );
    console.log("id", id);
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
    console.log("quoteToken.mint: " + quoteToken.mint);
    const id = this.findBoundPoolPda(
      memeMintKeypair.publicKey,
      quoteToken.mint,
      args.client.memechanProgram.programId
    );
    const poolSigner = BoundPoolClient.findSignerPda(
      id,
      args.client.memechanProgram.programId
    );
    console.log("poolSigner: " + poolSigner.toBase58());

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
    console.log("memeMint: " + memeMint.toBase58());

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
    const id = Keypair.generate();
    const user = input.user!;
    const payer = input.payer!;

    const pool = input.pool ?? this.id;
    const poolSignerPda = BoundPoolClient.findSignerPda(
      pool,
      this.client.memechanProgram.programId
    );
    const sol_in = input.quoteAmountIn;
    const meme_out = input.memeTokensOut;

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

    await this.client.memechanProgram.methods
      .swapY(new BN(sol_in), new BN(meme_out))
      .accounts({
        memeTicket: id.publicKey,
        owner: user.publicKey,
        pool: pool,
        poolSignerPda: poolSignerPda,
        quoteVault: this.quoteVault,
        userSol: userQuoteAcc,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user, id])
      .rpc({ skipPreflight: true, commitment: "confirmed" })
      .catch((e) => console.error(e));

    return new MemeTicket(id.publicKey, this.client);
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
      .swapY(inputAmountBN, minOutputBN)
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

    const methodArgs = {
      pool,
      signer: user,
      boundPoolSignerPda: this.findSignerPda(),
      memeTicket: adminTicketId,
      poolMemeVault: boundPoolInfo.memeReserve.vault,
      poolQuoteVault: boundPoolInfo.quoteReserve.vault,
      stakingMemeVault,
      stakingQuoteVault: stakingQuoteVault,
      quoteMint: this.quoteTokenMint,
      staking: stakingId,
      stakingPoolSignerPda: stakingSigner,
      feeVaultQuote: boundPoolInfo.feeVaultQuote,
      marketProgramId: PROGRAMIDS.OPENBOOK_MARKET,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      clock: SYSVAR_CLOCK_PUBKEY,
      rent: SYSVAR_RENT_PUBKEY,
      memeMint: boundPoolInfo.memeReserve.mint,
      ataProgram: ATA_PROGRAM_ID,
      user,
    };

    const initStakingPoolInstruction = await this.client.memechanProgram.methods
      .initStakingPool()
      .accounts(methodArgs)
      .instruction();

    tx.add(initStakingPoolInstruction);

    return { transaction: tx, stakingMemeVault, stakingQuoteVault };
  }

  public async initStakingPool(
    input: InitStakingPoolArgs
  ): Promise<InitStakingPoolResult> {
    const { transaction, stakingMemeVault, stakingQuoteVault } =
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

    return { stakingMemeVault, stakingQuoteVault };
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
          memeTicket: memeTicket.id,
          owner: user.publicKey,
          pool: pool,
          poolSigner: poolSignerPda,
          quoteVault: this.quoteVault,
          userSol: userSolAcc,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .instruction();

    tx.add(sellMemeTransactionInstruction);

    return tx;
  }

  public async getGoLiveTransaction(args: GetGoLiveTransactionArgs): Promise<{
    goLiveTransaction: Transaction;
    stakingId: PublicKey;
  }> {
    const {
      boundPoolInfo,
      user,
      feeDestinationWalletAddress,
      memeVault,
      quoteVault,
      transaction = new Transaction(),
    } = args;
    const stakingId = BoundPoolClient.findStakingPda(
      boundPoolInfo.memeReserve.mint,
      this.client.memechanProgram.programId
    );
    const stakingSigner = StakingPool.findSignerPda(
      stakingId,
      this.client.memechanProgram.programId
    );
    const baseTokenInfo = new Token(
      TOKEN_PROGRAM_ID,
      new PublicKey(boundPoolInfo.memeReserve.mint),
      MEMECHAN_MEME_TOKEN_DECIMALS
    );
    const quoteTokenInfo = MEMECHAN_QUOTE_TOKEN;

    // TODO: Put all the transactions into one (now they exceed trx size limit)
    // const createMarketInstructions = getCreateMarketInstructions(transactions);
    // createMarketTransaction.add(...createMarketInstructions);

    transaction.add(
      SystemProgram.transfer({
        fromPubkey: user.publicKey,
        toPubkey: stakingSigner,
        lamports: 2_000_000_000,
      })
    );

    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
      units: 250000,
    });

    transaction.add(modifyComputeUnits);

    const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 5000000,
    });

    transaction.add(addPriorityFee);

    const configAddress = new PublicKey(
      "9zSzfkYy6awexsHvmggeH36pfVUdDGyCcwmjT3AQPBj6"
    );

    const token0 = this.memeTokenMint;
    const token1 = QUOTE_MINT;

    const [auth] = await getAuthAddress(CPMM);
    const [poolAddress] = await getPoolAddress(
      configAddress,
      token0,
      token1,
      CPMM
    );
    const [lpMintAddress] = await getPoolLpMintAddress(poolAddress, CPMM);
    const [vault0] = await getPoolVaultAddress(poolAddress, token0, CPMM);
    const [vault1] = await getPoolVaultAddress(poolAddress, token1, CPMM);
    const [creatorLpTokenAddress] = await PublicKey.findProgramAddress(
      [
        stakingSigner.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        lpMintAddress.toBuffer(),
      ],
      ASSOCIATED_PROGRAM_ID
    );

    const [observationAddress] = await getOrcleAccountAddress(
      poolAddress,
      CPMM
    );

    const goLiveInstruction = await this.client.memechanProgram.methods
      .goLive()
      .accounts({
        signer: user.publicKey,
        poolMemeVault: memeVault,
        poolQuoteVault: quoteVault,
        quoteMint: this.quoteTokenMint,
        staking: stakingId,
        stakingPoolSignerPda: stakingSigner,
        raydiumLpMint: lpMintAddress,
        raydiumAmm: poolAddress,
        raydiumAmmAuthority: auth,
        raydiumMemeVault: vault0,
        raydiumQuoteVault: vault1,
        observationState: observationAddress,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        clock: SYSVAR_CLOCK_PUBKEY,
        rent: SYSVAR_RENT_PUBKEY,
        memeMint: boundPoolInfo.memeReserve.mint,
        ammConfig: configAddress,
        ataProgram: ATA_PROGRAM_ID,
        feeDestinationInfo: feeDestinationWalletAddress,
        userDestinationLpTokenAta: creatorLpTokenAddress,
        raydiumProgram: CPMM,
      })
      .instruction();

    transaction.add(goLiveInstruction);

    return {
      goLiveTransaction: transaction,
      stakingId,
    };
  }

  public async goLive2(args: GoLiveArgs): Promise<StakingPool> {
    console.log("goLive2 Begin");
    // Get needed transactions
    const { goLiveTransaction, stakingId } = await this.getGoLiveTransaction(
      args
    );

    console.log("goLive2 1");
    // Send transaction to go live
    const goLiveSignature = await sendAndConfirmTransaction(
      this.client.connection,
      goLiveTransaction,
      [args.user],
      {
        skipPreflight: true,
        commitment: "confirmed",
      }
    );
    console.log("go live signature:", goLiveSignature);

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

  public async goLive(
    input: GoLiveArgs
  ): Promise<[StakingPool, ApiPoolInfoV4]> {
    const user = input.user!;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const boundPoolInfo = input.boundPoolInfo as any;
    const stakingId = BoundPoolClient.findStakingPda(
      boundPoolInfo.memeReserve.mint,
      this.client.memechanProgram.programId
    );
    const stakingSigner = StakingPool.findSignerPda(
      stakingId,
      this.client.memechanProgram.programId
    );

    console.log("goLive.boundPoolInfo: " + JSON.stringify(boundPoolInfo));

    const baseTokenInfo = new Token(
      TOKEN_PROGRAM_ID,
      new PublicKey(boundPoolInfo.memeReserve.mint),
      MEMECHAN_MEME_TOKEN_DECIMALS
    );
    //const marketId = new PublicKey("AHZCwnUuiB3CUEyk2nybsU5c85WVDTHVP2UwuQwpVaR1");
    const quoteTokenInfo = MEMECHAN_QUOTE_TOKEN;

    const { txids: createMarketTxIds, marketId } = await createMarket({
      baseToken: baseTokenInfo,
      quoteToken: quoteTokenInfo,
      wallet: user.publicKey,
      signer: user,
      connection: this.client.connection,
    });

    console.log("createMarketTxIds: " + JSON.stringify(createMarketTxIds));

    const createMarkeLatestBH0 =
      await this.client.connection.getLatestBlockhash("confirmed");
    const createMarketTxResult =
      await this.client.connection.confirmTransaction(
        {
          signature: createMarketTxIds[0],
          blockhash: createMarkeLatestBH0.blockhash,
          lastValidBlockHeight: createMarkeLatestBH0.lastValidBlockHeight,
        },
        "confirmed"
      );

    if (createMarketTxResult.value.err) {
      console.error("createMarketTxResult", createMarketTxResult);
      throw new Error("createMarketTxResult failed");
    }

    console.log("marketId", marketId.toBase58());
    console.log("stakingId: " + stakingId.toBase58());

    // const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
    //   units: 300,
    // });

    // const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    //   microLamports: 20000,
    // });

    const transferTx = new Transaction().add(
      // modifyComputeUnits,
      // addPriorityFee,
      SystemProgram.transfer({
        fromPubkey: user.publicKey,
        toPubkey: stakingSigner,
        lamports: 2_000_000_000,
      })
    );

    const transferSignature = await sendAndConfirmTransaction(
      this.client.connection,
      transferTx,
      [user],
      {
        skipPreflight: true,
        commitment: "confirmed",
      }
    );

    console.log("transferSignature: " + transferSignature);

    const transferTxBH0 = await this.client.connection.getLatestBlockhash(
      "confirmed"
    );
    const transferTxSyncResult =
      await this.client.connection.confirmTransaction(
        {
          signature: transferSignature,
          blockhash: transferTxBH0.blockhash,
          lastValidBlockHeight: transferTxBH0.lastValidBlockHeight,
        },
        "confirmed"
      );

    if (transferTxSyncResult.value.err) {
      console.error(
        "transferTxSyncResult error: ",
        JSON.stringify(transferTxSyncResult)
      );
      throw new Error("transferTxSyncResult failed");
    } else {
      console.log(
        "transferTxSyncResult: " + JSON.stringify(transferTxSyncResult)
      );
    }
    const ammConfig = BoundPoolClient.getAssociatedConfigId({
      programId: CPMM,
    });
    const feeDestination = input.feeDestinationWalletAddress;
    const ammId = BoundPoolClient.getAssociatedId({
      programId: CPMM,
      configId: ammConfig,
      mint0: this.memeTokenMint,
      mint1: QUOTE_MINT,
    });
    const raydiumAmmAuthority = BoundPoolClient.getAssociatedAuthority({
      programId: CPMM,
    });

    const raydiumLpMint = BoundPoolClient.getAssociatedLpMint({
      programId: CPMM,
      marketId,
    });

    const raydiumMemeVault = BoundPoolClient.getAssociatedBaseVault({
      programId: CPMM,
      marketId,
    });
    const raydiumWsolVault = BoundPoolClient.getAssociatedQuoteVault({
      programId: CPMM,
      marketId,
    });
    const observationState = BoundPoolClient.getObservationAddr({
      programId: CPMM,
      poolState: ammId,
    });

    const userDestinationLpTokenAta = BoundPoolClient.getATAAddress(
      stakingSigner,
      raydiumLpMint,
      TOKEN_PROGRAM_ID
    ).publicKey;

    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
      units: 250000,
    });

    const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 5000000,
    });

    // const data = Buffer.from(
    //   Uint8Array.of(0, ...new BN(500000).toArray("le", 4))
    // );
    // const additionalComputeBudgetInstruction = new TransactionInstruction({
    //   keys: [],
    //   programId: new PublicKey("ComputeBudget111111111111111111111111111111"),
    //   data,
    // });

    try {
      const result = await this.client.memechanProgram.methods
        .goLive()
        .accounts({
          signer: user.publicKey,
          poolMemeVault: input.memeVault,
          poolQuoteVault: input.quoteVault,
          quoteMint: this.quoteTokenMint,
          staking: stakingId,
          stakingPoolSignerPda: stakingSigner,
          raydiumLpMint: raydiumLpMint,
          raydiumAmm: ammId,
          raydiumAmmAuthority: raydiumAmmAuthority.publicKey,
          raydiumMemeVault: raydiumMemeVault,
          raydiumQuoteVault: raydiumWsolVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          observationState,
          clock: SYSVAR_CLOCK_PUBKEY,
          rent: SYSVAR_RENT_PUBKEY,
          memeMint: boundPoolInfo.memeReserve.mint,
          ammConfig: ammConfig,
          ataProgram: ATA_PROGRAM_ID,
          feeDestinationInfo: feeDestination,
          userDestinationLpTokenAta: userDestinationLpTokenAta,
          raydiumProgram: CPMM,
        })
        .signers([user])
        .preInstructions([modifyComputeUnits, addPriorityFee])
        .rpc({ skipPreflight: true, commitment: "confirmed" });
      console.log("goLive Transaction successful:", result);

      return [
        await StakingPool.fromStakingPoolId({
          client: this.client,
          poolAccountAddressId: stakingId,
        }),
        await formatAmmKeysById(ammId.toBase58(), this.client.connection),
      ];
    } catch (error) {
      if (error instanceof AnchorError) {
        console.error("Error details:", error);
        if (error.logs) {
          error.logs.forEach((log) => console.log("Program log:", log));
        }
      } else {
        console.error("Unexpected error:", error);
      }

      throw error;
    }
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
