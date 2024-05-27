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
} from "@solana/web3.js";
import {
  airdrop,
  // memechan,
  // payer,
  // provider,
  admin,
  // solMint,
  // amm,
  sleep,
  findProgramAddress,
  provider,
  payer,
  memechan,
  adminKeypair,
} from "./helpers";

import { BN } from "@project-serum/anchor";
import { MemeTicket } from "./ticket";
import {
  SYSVAR_CLOCK_PUBKEY,
  SYSVAR_RENT_PUBKEY,
  Token,
  WSOL,
} from "@raydium-io/raydium-sdk";
import { createMarket } from "./raydium/openbook";
import {
  MEMECHAN_MEME_TOKEN_DECIMALS,
  MEMECHAN_QUOTE_TOKEN_DECIMALS,
  QUOTE_TOKEN_DECIMALS,
  // MEMECHAN_QUOTE_MINT,
  // MEMECHAN_QUOTE_TOKEN,
} from "./config";
import { ATA_PROGRAM_ID, PROGRAMIDS } from "./raydium/config";
import { getCreateMarketTransactions } from "./raydium/openBookCreateMarket";
import { sendTx } from "./util";
import { formatAmmKeysById } from "./raydium/formatAmmKeysById";

const RAYDIUM_PROGRAM_ID = new PublicKey(
  "HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8"
);
const OPENBOOK_PROGRAM_ID = new PublicKey(
  "EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj"
);
const SLERF_MINT = new PublicKey(
  "HX2pp5za2aBkrA5X5iTioZXcrpWb2q9DiaeWPW3qKMaw"
);

export interface NewTargetConfig {
  sender: Keypair;
  targetConfig: PublicKey;
  mint: PublicKey;
}

export interface NewPool {
  sender: Keypair;
  pool: PublicKey;
  memeMint: PublicKey;
  quoteVault: PublicKey;
  quoteMint: PublicKey;
  adminQuoteVault: PublicKey;
  memeVault: PublicKey;
  targetConfig: PublicKey;
  poolSigner: PublicKey;
}

export interface SwapYArgs {
  pool: PublicKey;
  quoteVault: PublicKey;
  userQuoteWallet: PublicKey;
  memeTicket: Keypair;
  owner: Keypair;
  poolSignerPda: PublicKey;
  coinInAmount: BN;
  coinXMinValue: BN;
}

export interface SwapXArgs {
  pool: PublicKey;
  memeTicket: PublicKey;
  userQuoteWallet: PublicKey;
  quoteVault: PublicKey;
  owner: Keypair;
  poolSignerPda: PublicKey;
  coinInAmount: BN;
  coinYMinValue: BN;
}

export interface CloseTicketArgs {
  ticket: PublicKey;
  owner: Keypair;
}

export interface InitStakingPoolArgs {
  signer: Keypair;
  pool: PublicKey;
  boundPoolSignerPda: PublicKey;
  poolMemeVault: PublicKey;
  poolQuoteVault: PublicKey;
  adminVaultQuote: PublicKey;
  memeMint: PublicKey;
  quoteMint: PublicKey;
  staking: PublicKey;
  stakingPoolSignerPda: PublicKey;
  stakingMemeVault: PublicKey;
  stakingQuoteVault: PublicKey;
  memeTicket: PublicKey;
}

export interface GoLiveArgs {
  signer: Keypair;
  staking: PublicKey;
  stakingPoolSignerPda: PublicKey;
  poolMemeVault: PublicKey;
  poolWsolVault: PublicKey;
  memeMint: PublicKey;
  quoteMint: PublicKey;
  openOrders: PublicKey;
  targetOrders: PublicKey;
  marketAccount: PublicKey;
  raydiumAmm: PublicKey;
  raydiumAmmAuthority: PublicKey;
  raydiumLpMint: PublicKey;
  raydiumMemeVault: PublicKey;
  raydiumQuoteVault: PublicKey;
  ammConfig: PublicKey;
  feeDestination: PublicKey;
  userDestinationLpTokenAta: PublicKey;
}

export interface UnstakeArgs {
  staking: PublicKey;
  memeTicket: PublicKey;
  userMeme: PublicKey;
  userQuote: PublicKey;
  memeVault: PublicKey;
  signer: Keypair;
  stakingSignerPda: PublicKey;
  releaseAmount: BN;
}

export interface AddFeesArgs {
  staking: PublicKey;
  memeVault: PublicKey;
  quoteVault: PublicKey;
  stakingSignerPda: PublicKey;
  stakingLpWallet: PublicKey;
  signer: Keypair;
  raydiumAmm: PublicKey;
  raydiumAmmAuthority: PublicKey;
  raydiumMemeVault: PublicKey;
  raydiumQuoteVault: PublicKey;
  raydiumLpMint: PublicKey;
  openOrders: PublicKey;
  targetOrders: PublicKey;
  marketAccount: PublicKey;
  marketEventQueue: PublicKey;
  marketCoinVault: PublicKey;
  marketPcVault: PublicKey;
  marketVaultSigner: PublicKey;
  marketBids: PublicKey;
  marketAsks: PublicKey;
}

export interface WithdrawFeesArgs {
  staking: PublicKey;
  memeTicket: PublicKey;
  userMeme: PublicKey;
  userQuote: PublicKey;
  memeVault: PublicKey;
  quoteVault: PublicKey;
  stakingSignerPda: PublicKey;
  signer: Keypair;
}

export class BoundPool {
  public stakingQuoteVault?: PublicKey;
  public stakingMemeVault?: PublicKey;
  public marketId?: PublicKey;

  private constructor(
    public id: PublicKey,
    public quoteVault: PublicKey,
    public memeVault: PublicKey,
    public adminVaultQuote: PublicKey,
    public memeMint: PublicKey,
    public quoteMint: PublicKey,
    public targetConfig: PublicKey,
    public adminKeypair: Keypair
  ) {
    //
  }

  public static async new(input: Partial<NewPool>): Promise<BoundPool> {
    const sender = input.sender ?? Keypair.generate();
    await airdrop(sender.publicKey);
    await airdrop(payer.publicKey);
    await airdrop(adminKeypair.publicKey);

    let memeMintKeypair = Keypair.generate();
    let quoteMintKeypair = Keypair.generate();

    const pool =
      input.pool ??
      this.findBoundPoolPubkey(
        memeMintKeypair.publicKey,
        quoteMintKeypair.publicKey
      );
    const poolSigner = BoundPool.boundPoolSignerPda(pool);

    // 1. Create Meme Mint
    const memeMint =
      input.memeMint ??
      (await (async () => {
        return createMint(
          provider.connection,
          payer,
          poolSigner,
          null,
          MEMECHAN_MEME_TOKEN_DECIMALS,
          memeMintKeypair
        );
      })());

    // 2. Get quote mint
    const quoteMint =
      input.quoteMint ??
      (await (async () => {
        return createMint(
          provider.connection,
          payer,
          adminKeypair.publicKey,
          null,
          MEMECHAN_QUOTE_TOKEN_DECIMALS,
          quoteMintKeypair
        );
      })());

    // 3. Create Pool Quote Vault
    const quoteVault =
      input.quoteVault ??
      (await (async () => {
        const quoteVaultKeypair = Keypair.generate();

        const quoteVault = await createAccount(
          provider.connection,
          payer,
          quoteMint,
          poolSigner,
          quoteVaultKeypair,
          { skipPreflight: true }
        );

        return quoteVault;
      })());

    // 4. Create AdminQuoteVault
    // If `adminSolPublicKey` is not passed in args, we need to find out, whether a quote account for an admin
    // already exists
    const adminQuoteVault =
      input.adminQuoteVault ??
      (await (async () => {
        const associatedToken = await getOrCreateAssociatedTokenAccount(
          provider.connection,
          payer,
          quoteMint,
          admin
          // associatedToken,
          // quoteToken.mint,
          // TOKEN_PROGRAM_ID,
          // ASSOCIATED_TOKEN_PROGRAM_ID
        );

        return associatedToken.address;
      })());

    // 5. Create Meme Quote Vault
    const memeVault =
      input.memeVault ??
      (await (async () => {
        const memeVaultKeypair = Keypair.generate();

        const memeVault = await createAccount(
          provider.connection,
          payer,
          memeMint,
          poolSigner,
          memeVaultKeypair
        );

        return memeVault;
      })());

    const targetConfig =
      input.targetConfig ?? this.targetConfigPubkey(quoteMint);

    await memechan.methods
      // .newTargetConfig(new BN(40000000000000))
      .newTargetConfig(new BN(40_000 * QUOTE_TOKEN_DECIMALS))
      .accounts({
        sender: adminKeypair.publicKey,
        targetConfig,
        mint: quoteMint,
        systemProgram: SystemProgram.programId,
      })
      .signers([adminKeypair])
      .rpc({ skipPreflight: true });

    await memechan.methods
      .newPool()
      .accounts({
        adminQuoteVault,
        memeVault,
        quoteVault,
        memeMint: memeMint,
        pool,
        poolSigner,
        sender: sender.publicKey,
        quoteMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        targetConfig,
      })
      .signers([sender])
      .rpc({ skipPreflight: true });

    return new BoundPool(
      pool,
      quoteVault,
      memeVault,
      adminQuoteVault,
      memeMint,
      quoteMint,
      targetConfig,
      adminKeypair
    );
  }

  public async fetch() {
    return memechan.account.boundPool.fetch(this.id);
  }

  public static findBoundPoolPubkey(
    memeMintPubkey: PublicKey,
    quoteMintPubkey: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from("bound_pool"),
        memeMintPubkey.toBytes(),
        quoteMintPubkey.toBytes(),
      ],
      memechan.programId
    )[0];
  }

  // Ok
  public static boundPoolSignerPda(poolId: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("signer"), poolId.toBytes()],
      memechan.programId
    )[0];
  }

  public static targetConfigPubkey(mint: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("config"), mint.toBytes()],
      memechan.programId
    )[0];
  }

  // Ok
  public signerPda(): PublicKey {
    return BoundPool.boundPoolSignerPda(this.id);
  }

  public stakingPubkey(): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("staking_pool"), this.memeMint.toBytes()],
      memechan.programId
    )[0];
  }

  public stakingSignerPda(stakingId: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("staking"), stakingId.toBytes()],
      memechan.programId
    )[0];
  }

  public adminMemeTicket(stakingId: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("admin_ticket"), stakingId.toBytes()],
      memechan.programId
    )[0];
  }

  // public signer(): PublicKey {
  //   return BoundPool.signerFrom(this.id);
  // }

  // public associatedPda(marketAccount: PublicKey, seed: string): PublicKey {
  //   const pda = PublicKey.findProgramAddressSync(
  //     [
  //       RAYDIUM_PROGRAM_ID.toBytes(),
  //       marketAccount.toBytes(),
  //       Buffer.from(TARGET_ASSOCIATED_SEED),
  //     ],
  //     RAYDIUM_PROGRAM_ID
  //   )[0];

  //   return pda;
  // }

  public static async airdropLiquidityTokens(
    mint: PublicKey,
    wallet: PublicKey,
    authority: Signer,
    amount: number = 1_000_000
  ) {
    return mintTo(provider.connection, payer, mint, wallet, authority, amount);
  }

  public async swapY(input: Partial<SwapYArgs>): Promise<MemeTicket> {
    const user = input.owner ?? Keypair.generate();
    await airdrop(user.publicKey);

    const pool = input.pool ?? this.id;
    const poolSignerPda = input.poolSignerPda ?? this.signerPda();
    const coinInAmount = input.coinInAmount ?? new BN(1 * 1e9);
    const coinXMinValue = input.coinXMinValue ?? new BN(1);

    const quoteVault = input.quoteVault ?? this.quoteVault;

    const userQuoteWallet =
      input.userQuoteWallet ??
      (await (async () => {
        const userQuoteWalletKeypair = Keypair.generate();

        const userQuoteWallet = await createAccount(
          provider.connection,
          payer,
          this.quoteMint,
          user.publicKey,
          userQuoteWalletKeypair
        );

        return userQuoteWallet;
      })());

    await mintTo(
      provider.connection,
      payer,
      this.quoteMint,
      userQuoteWallet,
      this.adminKeypair,
      coinInAmount.toNumber()
    );

    const memeTicketKeypair = input.memeTicket ?? Keypair.generate();

    await memechan.methods
      .swapY(new BN(coinInAmount), new BN(coinXMinValue))
      .accounts({
        pool,
        quoteVault,
        userQuoteWallet,
        memeTicket: memeTicketKeypair.publicKey,
        owner: user.publicKey,
        poolSignerPda: poolSignerPda,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user, memeTicketKeypair])
      .rpc({ skipPreflight: true });

    return new MemeTicket(memeTicketKeypair.publicKey);
  }

  public quoteMintInfo(): Token {
    const quoteTokenInfo: Token = new Token(
      TOKEN_PROGRAM_ID,
      this.quoteMint,
      MEMECHAN_QUOTE_TOKEN_DECIMALS,
      "SLERF",
      "SLERF"
    );

    return quoteTokenInfo;
  }

  public async swapX(input: Partial<SwapXArgs>): Promise<void> {
    const user = input.owner ?? Keypair.generate();
    await airdrop(user.publicKey);

    const pool = input.pool ?? this.id;
    const poolSigner = input.poolSignerPda ?? this.signerPda();
    const coinInAmount = input.coinInAmount ?? 9e6 * 1e6;
    const coinYMinValue = input.coinYMinValue ?? 1;

    const quoteVault = input.quoteVault ?? this.quoteVault;

    const memeTicket = input.memeTicket!;

    const userQuoteWallet =
      input.userQuoteWallet ??
      (await (async () => {
        const userQuoteWalletKeypair = Keypair.generate();

        const userQuoteWallet = await createAccount(
          provider.connection,
          payer,
          this.quoteMint,
          user.publicKey,
          userQuoteWalletKeypair
        );

        return userQuoteWallet;
      })());

    await memechan.methods
      .swapX(new BN(coinInAmount), new BN(coinYMinValue))
      .accounts({
        pool,
        memeTicket,
        userQuoteWallet,
        quoteVault,
        owner: user.publicKey,
        poolSigner,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc({ skipPreflight: true });
  }

  public async closeTicket(
    input: Partial<CloseTicketArgs>
  ): Promise<MemeTicket> {
    const user = input.owner ?? Keypair.generate();
    await airdrop(user.publicKey);

    const memeTicket = input.ticket!;

    await memechan.methods
      .closeTicket()
      .accounts({
        ticket: memeTicket,
        owner: user.publicKey,
      })
      .signers([user])
      .rpc({ skipPreflight: true });

    return;
  }

  public async addFees(input: Partial<AddFeesArgs>): Promise<MemeTicket> {
    const user = input.signer ?? Keypair.generate();
    await airdrop(user.publicKey);
    const staking = input.staking ?? this.stakingPubkey();
    const memeVault = input.memeVault ?? this.memeVault;
    const quoteVault = input.quoteVault ?? this.quoteVault;
    const stakingSigner =
      input.stakingSignerPda ?? this.stakingSignerPda(staking);

    const raydiumLpMint =
      input.raydiumLpMint ??
      this.getRaydiumLpMint({
        programId: PROGRAMIDS.AmmV4,
        marketId: this.marketId!,
      });

    const stakingLpWallet =
      input.stakingLpWallet ??
      (await (async () => {
        const stakingLpWalletKeypair = Keypair.generate();

        const stakingLpWallet = await createAccount(
          provider.connection,
          payer,
          raydiumLpMint,
          user.publicKey,
          stakingLpWalletKeypair
        );

        return stakingLpWallet;
      })());

    const raydiumAmm =
      input.raydiumAmm ??
      this.raydiumAmmPubkey({
        programId: PROGRAMIDS.AmmV4,
        marketId: this.marketId!,
      });
    const raydiumAmmAuthority =
      input.raydiumAmmAuthority ??
      this.raydiumAuthority({
        programId: PROGRAMIDS.AmmV4,
      }).publicKey;
    const openOrders =
      input.openOrders ??
      this.getAssociatedOpenOrders({
        programId: PROGRAMIDS.AmmV4,
        marketId: this.marketId!,
      });
    const targetOrders =
      input.targetOrders ??
      this.getAssociatedTargetOrders({
        programId: PROGRAMIDS.AmmV4,
        marketId: this.marketId!,
      });

    const raydiumMemeVault =
      input.raydiumMemeVault ??
      this.getRaydiumBaseVault({
        programId: PROGRAMIDS.AmmV4,
        marketId: this.marketId!,
      });
    const raydiumQuoteVault =
      input.raydiumQuoteVault ??
      this.getRaydiumQuoteVault({
        programId: PROGRAMIDS.AmmV4,
        marketId: this.marketId!,
      });

    const marketAccount = input.marketAccount ?? this.marketId!;

    const ammPool = await formatAmmKeysById(
      raydiumAmm.toBase58(),
      provider.connection
    );

    const marketEventQueue = input.marketEventQueue ?? ammPool.marketEventQueue;

    const marketAsks = input.marketAsks ?? ammPool.marketAsks;
    const marketBids = input.marketBids ?? ammPool.marketBids;
    const marketCoinVault = input.marketCoinVault ?? ammPool.marketBaseVault;
    const marketPcVault = input.marketPcVault ?? ammPool.marketQuoteVault;
    const marketVaultSigner =
      input.marketVaultSigner ?? ammPool.marketAuthority;

    await memechan.methods
      .addFees()
      .accounts({
        staking,
        memeVault,
        quoteVault,
        stakingSignerPda: stakingSigner,
        stakingLpWallet,
        signer: user.publicKey,
        raydiumAmm,
        raydiumAmmAuthority: raydiumAmmAuthority,
        raydiumMemeVault,
        raydiumQuoteVault,
        raydiumLpMint,
        openOrders,
        targetOrders,
        marketAccount,
        marketEventQueue,
        marketCoinVault,
        marketPcVault,
        marketVaultSigner,
        marketBids,
        marketAsks,
        tokenProgram: TOKEN_PROGRAM_ID,
        raydiumProgram: ammPool.programId,
        marketProgramId: ammPool.marketProgramId,
      })
      .signers([user])
      .rpc({ skipPreflight: true });

    return;
  }

  public async withdrawFees(input: Partial<WithdrawFeesArgs>): Promise<void> {
    const user = input.signer ?? Keypair.generate();
    await airdrop(user.publicKey);
    const staking = input.staking ?? this.stakingPubkey();

    const memeTicket = input.memeTicket!;

    const userMeme =
      input.userMeme ??
      (await (async () => {
        const userMemeKeypair = Keypair.generate();

        const userMeme = await createAccount(
          provider.connection,
          payer,
          this.memeMint,
          user.publicKey,
          userMemeKeypair
        );

        return userMeme;
      })());

    const userQuote =
      input.userQuote ??
      (await (async () => {
        const userQuoteKeypair = Keypair.generate();

        const userQuote = await createAccount(
          provider.connection,
          payer,
          this.memeMint,
          user.publicKey,
          userQuoteKeypair
        );

        return userQuote;
      })());

    const memeVault = input.quoteVault ?? this.stakingMemeVault;
    const quoteVault = input.quoteVault ?? this.stakingQuoteVault;

    const stakingSigner =
      input.stakingSignerPda ?? this.stakingSignerPda(staking);

    await memechan.methods
      .withdrawFees()
      .accounts({
        staking,
        memeTicket,
        userMeme,
        userQuote,
        memeVault,
        quoteVault,
        stakingSignerPda: stakingSigner,
        signer: user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc({ skipPreflight: true });
  }

  public async initStakingPool(
    input: Partial<InitStakingPoolArgs>
  ): Promise<MemeTicket> {
    const user = input.signer ?? Keypair.generate();
    await airdrop(user.publicKey);

    const pool = input.pool ?? this.id;
    const poolSigner = input.boundPoolSignerPda ?? this.signerPda();
    const poolMemeVault = input.poolMemeVault ?? this.memeVault;
    const poolQuoteVault = input.poolQuoteVault ?? this.quoteVault;
    const adminVaultQuote = input.adminVaultQuote ?? this.adminVaultQuote;
    const memeMint = input.memeMint ?? this.memeMint;
    const quoteMint = input.quoteMint ?? this.quoteMint;
    const staking = input.staking ?? this.stakingPubkey();
    const stakingSigner =
      input.stakingPoolSignerPda ?? this.stakingSignerPda(staking);

    const stakingMemeVault =
      input.stakingMemeVault ??
      (await (async () => {
        const stakingMemeVaultKeypair = Keypair.generate();

        const stakingMemeVault = await createAccount(
          provider.connection,
          payer,
          quoteMint,
          stakingSigner,
          stakingMemeVaultKeypair
        );

        return stakingMemeVault;
      })());

    const stakingQuoteVault =
      input.stakingQuoteVault ??
      (await (async () => {
        const stakingQuoteVaultKeypair = Keypair.generate();

        const quoteVault = await createAccount(
          provider.connection,
          payer,
          quoteMint,
          stakingSigner,
          stakingQuoteVaultKeypair
        );

        return quoteVault;
      })());

    const memeTicket = input.memeTicket ?? this.adminMemeTicket(staking);

    this.stakingMemeVault = stakingMemeVault;
    this.stakingQuoteVault = stakingQuoteVault;

    await memechan.methods
      .initStakingPool()
      .accounts({
        signer: user.publicKey,
        pool,
        boundPoolSignerPda: poolSigner,
        poolMemeVault,
        poolQuoteVault,
        adminVaultQuote,
        memeMint,
        quoteMint,
        staking,
        stakingPoolSignerPda: stakingSigner,
        stakingMemeVault,
        stakingQuoteVault,
        memeTicket,
        rent: SYSVAR_RENT_PUBKEY,
        clock: SYSVAR_CLOCK_PUBKEY,
        ataProgram: ATA_PROGRAM_ID,
        marketProgramId: PROGRAMIDS.OPENBOOK_MARKET,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc({ skipPreflight: true });

    return;
  }

  public async unstake(input: Partial<UnstakeArgs>): Promise<MemeTicket> {
    const user = input.signer ?? Keypair.generate();
    const staking = input.staking ?? this.stakingPubkey();
    const memeTicket = input.memeTicket!;

    const userMeme =
      input.userMeme ??
      (await (async () => {
        const userMemeKeypair = Keypair.generate();

        const userMeme = await createAccount(
          provider.connection,
          payer,
          this.memeMint,
          user.publicKey,
          userMemeKeypair
        );

        return userMeme;
      })());

    const userQuote =
      input.userQuote ??
      (await (async () => {
        const userQuoteKeypair = Keypair.generate();

        const userQuote = await createAccount(
          provider.connection,
          payer,
          this.memeMint,
          user.publicKey,
          userQuoteKeypair
        );

        return userQuote;
      })());

    const memeVault = input.memeVault ?? this.memeVault;
    const stakingSignerPda =
      input.stakingSignerPda ?? this.stakingSignerPda(staking);
    const releaseAmount = input.releaseAmount ?? new BN(100);

    await airdrop(user.publicKey);

    await memechan.methods
      .unstake(releaseAmount)
      .accounts({
        staking,
        memeTicket,
        userMeme,
        userQuote,
        memeVault,
        signer: user.publicKey,
        stakingSignerPda,
      })
      .signers([user])
      .rpc({ skipPreflight: true });

    return;
  }

  public raydiumAmmPubkey({
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
        Buffer.from("amm_associated_seed", "utf-8"),
      ],
      programId
    );
    return publicKey;
  }

  public raydiumAuthority({ programId }: { programId: PublicKey }) {
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

  public getAssociatedOpenOrders({
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

  public getRaydiumConfigId({ programId }: { programId: PublicKey }) {
    const { publicKey } = findProgramAddress(
      [Buffer.from("amm_config_account_seed", "utf-8")],
      programId
    );
    return publicKey;
  }

  public getAssociatedTargetOrders({
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

  public getRaydiumBaseVault({
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

  public getRaydiumQuoteVault({
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

  public getRaydiumLpMint({
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

  public getATAAddress(
    owner: PublicKey,
    mint: PublicKey,
    programId: PublicKey
  ) {
    return findProgramAddress(
      [owner.toBuffer(), programId.toBuffer(), mint.toBuffer()],
      new PublicKey(ATA_PROGRAM_ID)
    );
  }

  public async goLive(input: Partial<GoLiveArgs>): Promise<void> {
    const user = input.signer ?? Keypair.generate();
    await airdrop(user.publicKey);

    const memeMint = input.memeMint ?? this.memeMint;
    const quoteMint = input.quoteMint ?? this.quoteMint;

    const poolMemeVault = input.poolMemeVault ?? this.stakingMemeVault!;
    const poolQuoteVault = input.poolWsolVault ?? this.stakingQuoteVault!;

    const staking = input.staking ?? this.stakingPubkey();
    const stakingSigner =
      input.stakingPoolSignerPda ?? this.stakingSignerPda(staking);

    const baseTokenInfo = new Token(
      TOKEN_PROGRAM_ID,
      new PublicKey(this.memeMint),
      MEMECHAN_MEME_TOKEN_DECIMALS
    );

    const quoteTokenInfo: Token = new Token(
      TOKEN_PROGRAM_ID,
      quoteMint,
      MEMECHAN_QUOTE_TOKEN_DECIMALS,
      "SLERF",
      "SLERF"
    );

    const { marketId, transactions: createMarketTransactions } =
      await getCreateMarketTransactions({
        baseToken: baseTokenInfo,
        quoteToken: quoteTokenInfo,
        wallet: user.publicKey,
        signer: user,
        connection: provider.connection,
      });

    await sendTx(provider.connection, payer, createMarketTransactions, {
      skipPreflight: true,
    });

    await airdrop(stakingSigner);

    const feeDestination = input.feeDestination ?? Keypair.generate().publicKey;
    const ammId = this.raydiumAmmPubkey({
      programId: PROGRAMIDS.AmmV4,
      marketId,
    });
    const raydiumAmmAuthority = this.raydiumAuthority({
      programId: PROGRAMIDS.AmmV4,
    });
    const openOrders = this.getAssociatedOpenOrders({
      programId: PROGRAMIDS.AmmV4,
      marketId,
    });
    const targetOrders = this.getAssociatedTargetOrders({
      programId: PROGRAMIDS.AmmV4,
      marketId,
    });
    const ammConfig = this.getRaydiumConfigId({
      programId: PROGRAMIDS.AmmV4,
    });
    const raydiumLpMint = this.getRaydiumLpMint({
      programId: PROGRAMIDS.AmmV4,
      marketId,
    });
    const raydiumMemeVault = this.getRaydiumBaseVault({
      programId: PROGRAMIDS.AmmV4,
      marketId,
    });
    const raydiumWsolVault = this.getRaydiumQuoteVault({
      programId: PROGRAMIDS.AmmV4,
      marketId,
    });

    const userDestinationLpTokenAta = this.getATAAddress(
      stakingSigner,
      raydiumLpMint,
      TOKEN_PROGRAM_ID
    ).publicKey;

    this.marketId = marketId;

    await memechan.methods
      .goLive(raydiumAmmAuthority.nonce)
      .accounts({
        signer: user.publicKey,
        poolMemeVault,
        poolQuoteVault,
        quoteMint: quoteMint,
        staking: staking,
        stakingPoolSignerPda: stakingSigner,
        raydiumLpMint: raydiumLpMint,
        raydiumAmm: ammId,
        raydiumAmmAuthority: raydiumAmmAuthority.publicKey,
        raydiumMemeVault: raydiumMemeVault,
        raydiumQuoteVault: raydiumWsolVault,
        marketProgramId: PROGRAMIDS.OPENBOOK_MARKET,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        marketAccount: marketId,
        clock: SYSVAR_CLOCK_PUBKEY,
        rent: SYSVAR_RENT_PUBKEY,
        openOrders: openOrders,
        targetOrders: targetOrders,
        memeMint: memeMint,
        ammConfig: ammConfig,
        ataProgram: ATA_PROGRAM_ID,
        feeDestinationInfo: feeDestination,
        userDestinationLpTokenAta: userDestinationLpTokenAta,
        raydiumProgram: PROGRAMIDS.AmmV4,
      })
      .signers([user])
      .rpc({ skipPreflight: true });
  }

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
  //     .rpc({ skipPreflight: true });
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
  //     .rpc({ skipPreflight: true });
  // }

  // public async setSwapFee(permillion: number) {
  //   await amm.methods
  //     .setPoolSwapFee({
  //       permillion: new BN(permillion),
  //     })
  //     .accounts({ admin: this.admin.publicKey, pool: this.id.publicKey })
  //     .signers([this.admin])
  //     .rpc({ skipPreflight: true });
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
// }
