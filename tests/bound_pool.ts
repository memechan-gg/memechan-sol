import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  AuthorityType,
  closeAccount,
  createAccount,
  createMint,
  createNativeMint,
  createWrappedNativeAccount,
  getAccount,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  NATIVE_MINT,
  setAuthority,
  TOKEN_PROGRAM_ID,
  transfer,
} from "@solana/spl-token";
import {
  AccountMeta,
  PublicKey,
  Keypair,
  Signer,
  SystemProgram,
  ComputeBudgetProgram,
  Transaction,
  sendAndConfirmTransaction,
  SYSVAR_CLOCK_PUBKEY,
  SYSVAR_RENT_PUBKEY
} from "@solana/web3.js";
import {
  airdrop,
  memechan,
  payer,
  provider,
  admin,
  QUOTE_MINT,
  sleep,
  adminSigner,
  findProgramAddress,
} from "./helpers";
import { BN } from "@coral-xyz/anchor";
import { AmmPool } from "./pool";
import { Staking } from "./staking";
import { MemeTicket } from "./ticket";
import { Token, WSOL } from "@raydium-io/raydium-sdk";
import { createMarket } from "./raydium/openbook";
import { IDL } from "../target/types/memechan_sol";

export const RAYDIUM_PROGRAM_ID = new PublicKey("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8");
export const OPENBOOK_ID = new PublicKey("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj")
export const MEMECHAN_MEME_TOKEN_DECIMALS = 6;
export const FEE_DESTINATION_ID=new PublicKey("3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR")

export interface SwapYArgs {
  user: Keypair;
  pool: PublicKey;
  poolSignerPda: PublicKey;
  userSolAcc: PublicKey;
  solAmountIn: BN;
  memeTokensOut: BN;
}

export interface SwapXArgs {
  user: Keypair;
  pool: PublicKey;
  poolSignerPda: PublicKey;
  memeAmountIn: BN;
  solTokensOut: BN;
  userMemeTicket: MemeTicket;
  userSolAcc: PublicKey;
}

export interface GoLiveArgs {
  pool: PublicKey;
  staking: PublicKey;
  poolMemeVault: PublicKey;
  poolWsolVault: PublicKey;
  adminVaultSol: PublicKey;
  memeTicket: PublicKey;
  boundPoolSignerPda: PublicKey;
  stakingPoolSignerPda: PublicKey;
  memeMint: PublicKey;
  solMint: PublicKey;
  raydiumLpMint: PublicKey;
  signer: Keypair;
  raydiumAmm: PublicKey;
  raydiumAmmAuthority: PublicKey;
  raydiumMemeVault: PublicKey;
  raydiumWsolVault: PublicKey;
  ammConfig: PublicKey;
  feeDestination: PublicKey;
  userDestinationLpTokenAta: PublicKey;
  openOrders: PublicKey;
  targetOrders: PublicKey;
  marketAccount: PublicKey;
}

// pub rent: Sysvar<'info, Rent>,
// pub clock: Sysvar<'info, Clock>,
// pub ata_program: Program<'info, AssociatedToken>,
// pub market_program_id: Program<'info, OpenBook>,
// pub token_program: Program<'info, Token>,
// pub system_program: Program<'info, System>,

export interface InitStakingPoolArgs {
  pool?: PublicKey;
  user: Keypair;
  payer: Signer;
  boundPoolInfo: BoundPool;
}


export class BoundPool {
  private constructor(
    public id: PublicKey,
    public admin: Keypair,
    public solVault: PublicKey
  ) {
    //
  }

  public static async newTargetConfig(quoteMint: PublicKey, pool: PublicKey): Promise<void> {
    const configIdPDA = BoundPool.targerConfigPda(quoteMint);
    await memechan.methods.newTargetConfig(new BN(40_000_000_000_000))
    .accounts(
      {
        mint: quoteMint,
        sender: adminSigner.publicKey,
        systemProgram: SystemProgram.programId,
        targetConfig: configIdPDA,
      }
    ).signers([adminSigner])
    .rpc()
  }

  public static async new(): Promise<BoundPool> {
    const signer = Keypair.generate();
    await airdrop(signer.publicKey);

    const adminAuthority = admin;

    const memeMint = await createMint(
      provider.connection,
      payer,
      signer.publicKey,
      null,
      6
    );
    
    const id = BoundPool.findPoolID(memeMint, QUOTE_MINT);
    const poolSigner = BoundPool.signerFrom(id);

    await setAuthority(provider.connection, signer, memeMint, signer, AuthorityType.MintTokens, poolSigner)

    const adminSolVault = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        QUOTE_MINT,
        adminAuthority
      )
    ).address;

    const poolSolVaultid = Keypair.generate();
    const poolSolVault = await createAccount(
      provider.connection,
      payer,
      QUOTE_MINT,
      poolSigner,
      poolSolVaultid
    );

    const launchVaultid = Keypair.generate();
    const launchVault = await createAccount(
      provider.connection,
      payer,
      memeMint,
      poolSigner,
      launchVaultid
    );
    
    const targetConfig = BoundPool.targerConfigPda(QUOTE_MINT)
    const target = await memechan.account.targetConfig.fetchNullable(targetConfig);
    console.log(target)
    if (target === null) {
      await BoundPool.newTargetConfig(QUOTE_MINT, id)
    }

    await memechan.methods
      .newPool()
      .accounts({
        adminQuoteVault: adminSolVault,
        memeVault: launchVault,
        quoteVault: poolSolVault,
        memeMint: memeMint,
        pool: id,
        poolSigner: poolSigner,
        sender: signer.publicKey,
        targetConfig,
        quoteMint: QUOTE_MINT,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    return new BoundPool(id, signer, poolSolVault);
  }

  public async fetch() {
    return memechan.account.boundPool.fetch(this.id);
  }

  public static signerFrom(publicKey: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("signer"), publicKey.toBytes()],
      memechan.programId
    )[0];
  }


  public static findPoolID(memeMint: PublicKey, quoteMint: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("bound_pool"), memeMint.toBytes(), quoteMint.toBytes()],
      memechan.programId,
    )[0];
  }

  public signer(): PublicKey {
    return BoundPool.signerFrom(this.id);
  }

  public signerPda(): PublicKey {
    return BoundPool.signerFrom(this.id);
  }

  public static targerConfigPda(mint: PublicKey) {
    return BoundPool.findPDAGeneric("config", mint)
  }

  public static findPDAGeneric(pref: string, pubkey: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(pref), pubkey.toBytes()],
      memechan.programId,
    )[0];
  }

  public associatedPda(marketAccount: PublicKey, seed: string): [PublicKey, number] {
    const [pda, nom] = PublicKey.findProgramAddressSync(
      [
        RAYDIUM_PROGRAM_ID.toBytes(),
        marketAccount.toBytes(),
        Buffer.from(seed),
      ],
      RAYDIUM_PROGRAM_ID
    );

    return [pda, nom];
  }

  public static async airdropLiquidityTokens(
    mint: PublicKey,
    wallet: PublicKey,
    authority: Signer,
    amount: number = 1_000_000
  ) {
    return mintTo(provider.connection, payer, mint, wallet, authority, amount);
  }

  public async swap_y(input: Partial<SwapYArgs>): Promise<MemeTicket> {
    const user = input.user ?? Keypair.generate();
    await airdrop(user.publicKey);

    const id = Keypair.generate();

    const pool = input.pool ?? this.id;
    const poolSignerPda = input.poolSignerPda ?? this.signerPda();
    const quote_in = input.solAmountIn ?? new BN(1 * 1e9);
    const meme_out = input.memeTokensOut ?? new BN(1);

    const userQuoteAcc = (await getOrCreateAssociatedTokenAccount(provider.connection, user, QUOTE_MINT, user.publicKey))
      
    await mintTo(provider.connection, adminSigner, QUOTE_MINT, userQuoteAcc.address, admin, quote_in.toNumber())

    await memechan.methods
      .swapY(new BN(quote_in), new BN(meme_out))
      .accounts({
        memeTicket: id.publicKey,
        owner: user.publicKey,
        pool: pool,
        poolSignerPda: poolSignerPda,
        quoteVault: this.solVault,
        userSol: userQuoteAcc.address,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user, id])
      .rpc();

    return new MemeTicket(id.publicKey);
  }
/*
  public async swap_x(input: Partial<SwapXArgs>): Promise<void> {
    const user = input.user;

    const pool = input.pool ?? this.id;
    const poolSigner = input.poolSignerPda ?? this.signerPda();
    const meme_in = input.memeAmountIn ?? 9e6 * 1e6;
    const sol_out = input.solTokensOut ?? 1;

    const memeTicket = input.userMemeTicket;
    const userSolAcc = input.userSolAcc;

    await memechan.methods
      .swapX(new BN(meme_in), new BN(sol_out))
      .accounts({
        memeTicket: memeTicket.id,
        owner: user.publicKey,
        pool: pool,
        poolSigner,
        solVault: this.solVault,
        userSol: userSolAcc,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();
  }
*/

static getATAAddress(owner: PublicKey, mint: PublicKey, programId: PublicKey) {
  return findProgramAddress([owner.toBuffer(), programId.toBuffer(), mint.toBuffer()], new PublicKey(ASSOCIATED_TOKEN_PROGRAM_ID));
}

static getAssociatedId({ programId, marketId }: { programId: PublicKey; marketId: PublicKey }) {
  const { publicKey } = findProgramAddress(
    [programId.toBuffer(), marketId.toBuffer(), Buffer.from("amm_associated_seed", "utf-8")],
    programId,
  );
  return publicKey;
}

static getAssociatedAuthority({ programId }: { programId: PublicKey }) {
  return findProgramAddress(
    // new Uint8Array(Buffer.from('amm authority'.replace('\u00A0', ' '), 'utf-8'))
    [Buffer.from([97, 109, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121])],
    programId,
  );
}

static getAssociatedBaseVault({ programId, marketId }: { programId: PublicKey; marketId: PublicKey }) {
  const { publicKey } = findProgramAddress(
    [programId.toBuffer(), marketId.toBuffer(), Buffer.from("coin_vault_associated_seed", "utf-8")],
    programId,
  );
  return publicKey;
}

static getAssociatedQuoteVault({ programId, marketId }: { programId: PublicKey; marketId: PublicKey }) {
  const { publicKey } = findProgramAddress(
    [programId.toBuffer(), marketId.toBuffer(), Buffer.from("pc_vault_associated_seed", "utf-8")],
    programId,
  );
  return publicKey;
}

static getAssociatedLpMint({ programId, marketId }: { programId: PublicKey; marketId: PublicKey }) {
  const { publicKey } = findProgramAddress(
    [programId.toBuffer(), marketId.toBuffer(), Buffer.from("lp_mint_associated_seed", "utf-8")],
    programId,
  );
  return publicKey;
}

static getAssociatedLpVault({ programId, marketId }: { programId: PublicKey; marketId: PublicKey }) {
  const { publicKey } = findProgramAddress(
    [programId.toBuffer(), marketId.toBuffer(), Buffer.from("temp_lp_token_associated_seed", "utf-8")],
    programId,
  );
  return publicKey;
}

static getAssociatedTargetOrders({ programId, marketId }: { programId: PublicKey; marketId: PublicKey }) {
  const { publicKey } = findProgramAddress(
    [programId.toBuffer(), marketId.toBuffer(), Buffer.from("target_associated_seed", "utf-8")],
    programId,
  );
  return publicKey;
}

static getAssociatedWithdrawQueue({ programId, marketId }: { programId: PublicKey; marketId: PublicKey }) {
  const { publicKey } = findProgramAddress(
    [programId.toBuffer(), marketId.toBuffer(), Buffer.from("withdraw_associated_seed", "utf-8")],
    programId,
  );
  return publicKey;
}

static getAssociatedOpenOrders({ programId, marketId }: { programId: PublicKey; marketId: PublicKey }) {
  const { publicKey } = findProgramAddress(
    [programId.toBuffer(), marketId.toBuffer(), Buffer.from("open_order_associated_seed", "utf-8")],
    programId,
  );
  return publicKey;
}

static getAssociatedConfigId({ programId }: { programId: PublicKey }) {
  const { publicKey } = findProgramAddress([Buffer.from("amm_config_account_seed", "utf-8")], programId);
  return publicKey;
}

public static findStakingPda(memeMintPubkey: PublicKey, memechanProgramId: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("staking_pool"), memeMintPubkey.toBytes()],
    memechanProgramId,
  )[0];
}

public static findMemeTicketPda(stakingPubKey: PublicKey, memechanProgramId: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("admin_ticket"), stakingPubKey.toBytes()],
    memechanProgramId,
  )[0];
}

public static findSignerPda(publicKey: PublicKey, memechanProgramId: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync([Buffer.from("signer"), publicKey.toBytes()], memechanProgramId)[0];
}

public findSignerPda(): PublicKey {
  return BoundPool.findSignerPda(this.id, memechan.programId);
}

public async slowInitStakingPool(input: Partial<InitStakingPoolArgs>) {
  const user = input.user!;
  const pool = input.pool ?? this.id;

  //console.log("initStakingPool fetch: " + this.id.toBase58());
  //const boundPoolInfo = await this.fetch();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const boundPoolInfo = input.boundPoolInfo as any;

  console.log("initStakingPool.boundPoolInfo: " + JSON.stringify(boundPoolInfo));

  const stakingId = BoundPool.findStakingPda(
    boundPoolInfo.memeReserve.mint,
    memechan.programId,
  );
  const stakingSigner = Staking.findSignerPda(stakingId, memechan.programId);

  const adminTicketId = BoundPool.findMemeTicketPda(stakingId, memechan.programId);

  const stakingPoolQuoteVaultid = Keypair.generate();
  const stakingQuoteVault = await createAccount(
    provider.connection,
    user,
    QUOTE_MINT,
    stakingSigner,
    stakingPoolQuoteVaultid,
    { skipPreflight: true, commitment: "confirmed" },
  );

  const stakingMemeVaultid = Keypair.generate();
  const stakingMemeVault = await createAccount(
    provider.connection,
    user,
    boundPoolInfo.memeReserve.mint,
    stakingSigner,
    stakingMemeVaultid,
    { skipPreflight: true, commitment: "confirmed" },
  );

  try {
    const methodArgs = {
      pool: pool,
      signer: user.publicKey,
      boundPoolSignerPda: this.findSignerPda(),
      memeTicket: adminTicketId,
      poolMemeVault: boundPoolInfo.memeReserve.vault,
      poolQuoteVault: boundPoolInfo.quoteReserve.vault,
      stakingMemeVault: stakingMemeVault,
      stakingQuoteVault: stakingQuoteVault,
      quoteMint: QUOTE_MINT,
      staking: stakingId,
      stakingPoolSignerPda: stakingSigner,
      adminVaultQuote: boundPoolInfo.adminVaultQuote,
      marketProgramId: OPENBOOK_ID,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      clock: SYSVAR_CLOCK_PUBKEY,
      rent: SYSVAR_RENT_PUBKEY,
      memeMint: boundPoolInfo.memeReserve.mint,
      ataProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      user: user,
    };

    const result = await memechan.methods
      .initStakingPool()
      .accounts(methodArgs)
      .signers([methodArgs.user])
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log("initStakingPool Final result:", result);

    return { stakingMemeVault, stakingQuoteVault };
  } catch (error) {
    console.error("Failed to initialize staking pool:", error);
  }

  return { stakingMemeVault, stakingQuoteVault };
}

  public async go_live(
    input: Partial<GoLiveArgs>
  ): Promise<[AmmPool, Staking]> {
    const signer = input.signer ?? Keypair.generate();
    // await airdrop(signer.publicKey);

    // // Bonding Pool
    // const pool = input.pool ?? this.id;
    // const poolSigner = input.boundPoolSignerPda ?? this.signerPda();

    // const stakingId = Keypair.generate().publicKey;
    // const stakingSigner =
    //   input.stakingPoolSignerPda ?? Staking.signerFrom(stakingId);

   // const poolInfo = await memechan.account.boundPool.fetch(pool);

    const user = input.signer??signer;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const boundPoolInfo = (await memechan.account.boundPool.fetch(this.id));
    const stakingId = BoundPool.findStakingPda(
      boundPoolInfo.memeReserve.mint,
      memechan.programId,
    );
    const stakingSigner = Staking.findSignerPda(stakingId, memechan.programId);

    console.log("goLive.boundPoolInfo: " + JSON.stringify(boundPoolInfo));

    const baseTokenInfo = new Token(
      TOKEN_PROGRAM_ID,
      new PublicKey(boundPoolInfo.memeReserve.mint),
      MEMECHAN_MEME_TOKEN_DECIMALS,
    );
    //const marketId = new PublicKey("AHZCwnUuiB3CUEyk2nybsU5c85WVDTHVP2UwuQwpVaR1");
    const quoteTokenInfo = QUOTE_MINT;

    const { txids: createMarketTxIds, marketId } = await createMarket({
      baseToken: baseTokenInfo,
      quoteToken: quoteTokenInfo,
      wallet: user.publicKey,
      signer: user,
      connection: provider.connection,
    });

    console.log("createMarketTxIds: " + JSON.stringify(createMarketTxIds));

    const createMarkeLatestBH0 = await provider.connection.getLatestBlockhash("confirmed");
    const createMarketTxResult = await provider.connection.confirmTransaction(
      {
        signature: createMarketTxIds[0],
        blockhash: createMarkeLatestBH0.blockhash,
        lastValidBlockHeight: createMarkeLatestBH0.lastValidBlockHeight,
      },
      "confirmed",
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
      }),
    );

    const transferSignature = await sendAndConfirmTransaction(provider.connection, transferTx, [user], {
      skipPreflight: true,
      commitment: "confirmed",
    });

    console.log("transferSignature: " + transferSignature);

    const transferTxBH0 = await provider.connection.getLatestBlockhash("confirmed");
    const transferTxSyncResult = await provider.connection.confirmTransaction(
      {
        signature: transferSignature,
        blockhash: transferTxBH0.blockhash,
        lastValidBlockHeight: transferTxBH0.lastValidBlockHeight,
      },
      "confirmed",
    );

    if (transferTxSyncResult.value.err) {
      console.error("transferTxSyncResult error: ", JSON.stringify(transferTxSyncResult));
      throw new Error("transferTxSyncResult failed");
    } else {
      console.log("transferTxSyncResult: " + JSON.stringify(transferTxSyncResult));
    }

    const feeDestination = new PublicKey(FEE_DESTINATION_ID);
    const ammId = BoundPool.getAssociatedId({ programId: RAYDIUM_PROGRAM_ID, marketId });
    const raydiumAmmAuthority = BoundPool.getAssociatedAuthority({ programId: RAYDIUM_PROGRAM_ID });
    const openOrders = BoundPool.getAssociatedOpenOrders({ programId: RAYDIUM_PROGRAM_ID, marketId });
    const targetOrders = BoundPool.getAssociatedTargetOrders({ programId: RAYDIUM_PROGRAM_ID, marketId });
    const ammConfig = BoundPool.getAssociatedConfigId({ programId: RAYDIUM_PROGRAM_ID });
    const raydiumLpMint = BoundPool.getAssociatedLpMint({ programId: RAYDIUM_PROGRAM_ID, marketId });

    const raydiumMemeVault = BoundPool.getAssociatedBaseVault({ programId: RAYDIUM_PROGRAM_ID, marketId });
    const raydiumWsolVault = BoundPool.getAssociatedQuoteVault({ programId: RAYDIUM_PROGRAM_ID, marketId });

    const userDestinationLpTokenAta = BoundPool.getATAAddress(
      stakingSigner,
      raydiumLpMint,
      TOKEN_PROGRAM_ID,
    ).publicKey;

    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
      units: 250000,
    });

    // const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    //   microLamports: 5000000,
    // });

    // const data = Buffer.from(
    //   Uint8Array.of(0, ...new BN(500000).toArray("le", 4))
    // );
    // const additionalComputeBudgetInstruction = new TransactionInstruction({
    //   keys: [],
    //   programId: new PublicKey("ComputeBudget111111111111111111111111111111"),
    //   data,
    // });


    const result = memechan.methods
        .goLive(raydiumAmmAuthority.nonce)
        .accounts({
          signer: user.publicKey,
          poolMemeVault: input.poolMemeVault,
          poolQuoteVault: input.poolWsolVault,
          quoteMint: QUOTE_MINT,
          staking: stakingId,
          stakingPoolSignerPda: stakingSigner,
          raydiumLpMint: raydiumLpMint,
          raydiumAmm: ammId,
          raydiumAmmAuthority: raydiumAmmAuthority.publicKey,
          raydiumMemeVault: raydiumMemeVault,
          raydiumQuoteVault: raydiumWsolVault,
          marketProgramId: OPENBOOK_ID,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          marketAccount: marketId,
          clock: SYSVAR_CLOCK_PUBKEY,
          rent: SYSVAR_RENT_PUBKEY,
          openOrders: openOrders,
          targetOrders: targetOrders,
          memeMint: boundPoolInfo.memeReserve.mint,
          ammConfig: ammConfig,
          ataProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          feeDestinationInfo: feeDestination,
          userDestinationLpTokenAta: userDestinationLpTokenAta,
          raydiumProgram: RAYDIUM_PROGRAM_ID,
        })
        .signers([user])
        .preInstructions([modifyComputeUnits])
        .rpc({ skipPreflight: true, commitment: "confirmed" });

    return [
      new AmmPool(ammId),
      new Staking(stakingId),
    ];
  }
/*
  // public async redeemLiquidity(
  //   input: Partial<RedeemLiquidityArgs>
  // ): Promise<void> {
  //   const user = input.user ?? Keypair.generate();
  //   const pool = input.pool ?? this.id.publicKey;
  //   const poolSigner = input.poolSigner ?? this.signerPda();
  //   const lpMint = input.lpMint ?? (await this.fetch()).mint;
  //   const lpTokenWallet =
  //     input.lpTokenWallet ??
  //     (await createAccount(provider.connection, payer, lpMint, user.publicKey));

  //   const defineMinAmountTokens = async () => {
  //     const fetchPool = await this.fetch();
  //     const mint1 = fetchPool.reserves[0].mint;
  //     const mint2 = fetchPool.reserves[1].mint;

  //     const t: [PublicKey, { amount: BN }][] = [];
  //     t.push([mint1, { amount: new BN(0) }]);
  //     t.push([mint2, { amount: new BN(0) }]);

  //     return t;
  //   };

  //   const minAmountTokens =
  //     input.minAmountTokens ?? (await defineMinAmountTokens());

  //   const lpTokensToBurn = input.lpTokensToBurn ?? 100;

  //   const getVaultsAndWallets = async () => {
  //     const fetchPool = await this.fetch();

  //     const firstVault = fetchPool.reserves[0].vault;
  //     const secondVault = fetchPool.reserves[1].vault;

  //     const firstMint = fetchPool.reserves[0].mint;
  //     const secondMint = fetchPool.reserves[1].mint;

  //     const firstVaultAccount = await getAccount(
  //       provider.connection,
  //       firstVault
  //     );
  //     const secondVaultAccount = await getAccount(
  //       provider.connection,
  //       secondVault
  //     );

  //     const firstWalletAccount = await createAccount(
  //       provider.connection,
  //       payer,
  //       firstMint,
  //       user.publicKey
  //     );
  //     const secondWalletAccount = await createAccount(
  //       provider.connection,
  //       payer,
  //       secondMint,
  //       user.publicKey
  //     );

  //     return [
  //       {
  //         isSigner: false,
  //         isWritable: true,
  //         pubkey: firstVaultAccount.address,
  //       },
  //       {
  //         isSigner: false,
  //         isWritable: true,
  //         pubkey: firstWalletAccount,
  //       },
  //       {
  //         isSigner: false,
  //         isWritable: true,
  //         pubkey: secondVaultAccount.address,
  //       },
  //       {
  //         isSigner: false,
  //         isWritable: true,
  //         pubkey: secondWalletAccount,
  //       },
  //     ];
  //   };

  //   const vaultsAndWallets =
  //     input.vaultsAndWallets ?? (await getVaultsAndWallets());

  //   await amm.methods
  //     .redeemLiquidity({ amount: new BN(lpTokensToBurn) }, minAmountTokens)
  //     .accounts({
  //       user: user.publicKey,
  //       pool,
  //       poolSigner,
  //       lpMint,
  //       lpTokenWallet,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     })
  //     .remainingAccounts(vaultsAndWallets)
  //     .signers([user])
  //     .rpc();
  // }

  // public async swap(
  //   user: Keypair,
  //   sellWallet: PublicKey,
  //   buyWallet: PublicKey,
  //   sell: number,
  //   minBuy: number
  // ) {
  //   const pool = await this.fetch();
  //   const getVaultOfWallet = async (wallet: PublicKey) => {
  //     const { mint } = await getAccount(provider.connection, wallet);
  //     const reserves = pool.reserves as any[];
  //     return reserves.find((r) => r.mint.toBase58() === mint.toBase58()).vault;
  //   };

  //   await amm.methods
  //     .swap({ amount: new BN(sell) }, { amount: new BN(minBuy) })
  //     .accounts({
  //       user: user.publicKey,
  //       discount: discountAddress(user.publicKey),
  //       sellWallet,
  //       sellVault: await getVaultOfWallet(sellWallet),
  //       buyWallet,
  //       buyVault: await getVaultOfWallet(buyWallet),
  //       pool: this.id.publicKey,
  //       poolSigner: this.signerPda(),
  //       programTollWallet: pool.programTollWallet,
  //       lpMint: pool.mint,
  //     })
  //     .signers([user])
  //     .rpc();
  // }

  // public async setSwapFee(permillion: number) {
  //   await amm.methods
  //     .setPoolSwapFee({
  //       permillion: new BN(permillion),
  //     })
  //     .accounts({ admin: this.admin.publicKey, pool: this.id.publicKey })
  //     .signers([this.admin])
  //     .rpc();
  // }
}

// Pubkey::find_program_address(
//   &[
//       &info_id.to_bytes(),
//       &market_address.to_bytes(),
//       &associated_seed,
//   ],
//   program_id,
// )

// public static derivePda(programId: PublicKey, publicKey: PublicKey): PublicKey {
//   return PublicKey.findProgramAddressSync(
//     [Buffer.from("signer"), publicKey.toBytes()],
//     memechan.programId
//   )[0];
// }*/
}