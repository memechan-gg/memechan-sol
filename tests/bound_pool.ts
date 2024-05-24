import {
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
  memechan,
  payer,
  provider,
  admin,
  solMint,
  sleep,
} from "./helpers";
import { BN } from "@project-serum/anchor";
import { AmmPool } from "./pool";
import { Staking } from "./staking";
import { MemeTicket } from "./ticket";
import { Token, WSOL } from "@raydium-io/raydium-sdk";
import { createMarket } from "./raydium/openbook";

const RAYDIUM_PROGRAM_ID = new PublicKey("TODO");

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

export class BoundPool {
  private constructor(
    public id: PublicKey,
    public admin: Keypair,
    public solVault: PublicKey
  ) {
    //
  }

  public static async new(): Promise<BoundPool> {
    const id = Keypair.generate();

    const signer = Keypair.generate();
    await airdrop(signer.publicKey);

    const poolSigner = BoundPool.signerFrom(id.publicKey);

    const adminAuthority = admin;

    const memeMint = await createMint(
      provider.connection,
      payer,
      poolSigner,
      null,
      6
    );

    const adminSolVault = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        solMint,
        adminAuthority
      )
    ).address;

    const poolSolVaultid = Keypair.generate();
    const poolSolVault = await createAccount(
      provider.connection,
      payer,
      solMint,
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
    
    const targetConfig = BoundPool.targerConfigPda(id.publicKey);

    await memechan.methods
      .newPool()
      .accounts({
        adminQuoteVault: adminSolVault,
        memeVault: launchVault,
        quoteVault: poolSolVault,
        memeMint: memeMint,
        pool: id.publicKey,
        poolSigner: poolSigner,
        sender: signer.publicKey,
        targetConfig,
        quoteMint: solMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer, id])
      .rpc();

    return new BoundPool(id.publicKey, signer, poolSolVault);
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

  public signer(): PublicKey {
    return BoundPool.signerFrom(this.id);
  }

  public signerPda(): PublicKey {
    return BoundPool.signerFrom(this.id);
  }

  public static targerConfigPda(pool: PublicKey) {
    return BoundPool.findPDAGeneric("target_config", pool, memechan.programId)
  }

  public static findPDAGeneric(pref: string, pubkey: PublicKey, memechanProgramId: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(pref), pubkey.toBytes()],
      memechanProgramId,
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
/*
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
    const sol_in = input.solAmountIn ?? 1 * 1e9;
    const meme_out = input.memeTokensOut ?? 1;

    const userSolAcc =
      input.userSolAcc ??
      (await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        500 * 10e9
      ));

    await memechan.methods
      .swapY(new BN(sol_in), new BN(meme_out))
      .accounts({
        memeTicket: id.publicKey,
        owner: user.publicKey,
        pool: pool,
        poolSignerPda: poolSignerPda,
        quoteVault: this.solVault,
        userSol: userSolAcc,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user, id])
      .rpc();

    return new MemeTicket(id.publicKey);
  }

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

  public async go_live(
    input: Partial<GoLiveArgs>
  ): Promise<[AmmPool, Staking]> {
    const signer = input.signer ?? Keypair.generate();
    await airdrop(signer.publicKey);

    // Bonding Pool
    const pool = input.pool ?? this.id;
    const poolSigner = input.boundPoolSignerPda ?? this.signerPda();

    const stakingId = Keypair.generate().publicKey;
    const stakingSigner =
      input.stakingPoolSignerPda ?? Staking.signerFrom(stakingId);

    const poolInfo = await memechan.account.boundPool.fetch(pool);

    const poolMemeVault = input.poolMemeVault ?? poolInfo.poolMemeVault;
    const poolWsolVault = input.poolMemeVault ?? poolInfo.solReserve.mint;
    const adminVaultSol = input.poolMemeVault ?? poolInfo.adminVaultSol;

    const memeTicket = input.memeTicket ?? Keypair.generate().publicKey;

    const memeMint = input.memeMint ?? poolInfo.memeMint;
    const solMint = input.solMint ?? poolInfo.solReserve.mint;

    const Meme: Token = new Token(
      TOKEN_PROGRAM_ID,
      memeMint,
      Token.WSOL.decimals
    );
    const WSOL: Token = new Token(
      TOKEN_PROGRAM_ID,
      solMint,
      Token.WSOL.decimals
    );

    // Openbook
    let { txids, marketId: marketAccount } = await createMarket({
      baseToken: Meme,
      quoteToken: WSOL,
      wallet: signer,
      connection: provider.connection,
    });

    const openOrders = this.associatedPda(
      marketAccount,
      OPEN_ORDER_ASSOCIATED_SEED
    );

    const targetOrders = this.associatedPda(
      marketAccount,
      TARGET_ASSOCIATED_SEED
    );

    // Raydium
    // TODO..

    // signer: Keypair;
    // pool: PublicKey;
    // staking: PublicKey;
    // boundPoolSignerPda: PublicKey;
    // stakingPoolSignerPda: PublicKey;
    // poolMemeVault: PublicKey;
    // poolWsolVault: PublicKey;
    // adminVaultSol: PublicKey;
    // memeTicket: PublicKey;
    // memeMint: PublicKey;
    // solMint: PublicKey;

    // raydiumLpMint: PublicKey; <-- here!
    // raydiumAmm: PublicKey;
    // raydiumAmmAuthority: PublicKey;
    // raydiumMemeVault: PublicKey;
    // raydiumWsolVault: PublicKey;
    // ammConfig: PublicKey;
    // feeDestination: PublicKey;
    // userDestinationLpTokenAta: PublicKey;

    const vaults = await Promise.all(
      [poolInfo.memeMint, poolInfo.solReserve.mint].map(async (mint) => {
        const kp = Keypair.generate();
        await createAccount(
          provider.connection,
          payer,
          mint,
          ammPoolSigner,
          kp
        );
        return {
          isSigner: false,
          isWritable: true,
          pubkey: kp.publicKey,
        };
      })
    );

    const ammId = Keypair.generate();

    const ammPoolSigner = AmmPool.signerFrom(ammId.publicKey);

    const adminTicketId = Keypair.generate();

    const lpMint = await createMint(
      provider.connection,
      signer,
      ammPoolSigner,
      null,
      9
    );

    const lpTokenWalletId = Keypair.generate();
    const lpTokenWallet = await createAccount(
      provider.connection,
      signer,
      lpMint,
      poolSigner,
      lpTokenWalletId
    );

    const stakingMemeVaultId = Keypair.generate();

    const stakingMemeVault = await createAccount(
      provider.connection,
      payer,
      poolInfo.memeMint,
      stakingSigner,
      stakingMemeVaultId
    );

    const nkey = Keypair.generate();
    const userSolAcc = await createWrappedNativeAccount(
      provider.connection,
      payer,
      signer.publicKey,
      1e9,
      nkey
    );

    await closeAccount(
      provider.connection,
      payer,
      userSolAcc,
      stakingSigner,
      signer
    );

    await sleep(1000);

    await memechan.methods
      .goLive()
      .accounts({
        pool: pool,
        signer: user.publicKey,
        adminVaultSol: poolInfo.adminVaultSol,
        boundPoolSignerPda: poolSigner,
        lpTokenWallet,
        launchTokenVault: poolInfo.launchTokenVault,
        memeMint: poolInfo.memeMint,
        memeTicket: adminTicketId.publicKey,
        solMint: NATIVE_MINT,
        poolWsolVault: poolInfo.solReserve.vault,
        staking: stakingId.publicKey,
        stakingPoolSignerPda: stakingSigner,
        aldrinLpMint: lpMint,
        aldrinPoolAcc: ammId.publicKey,
        aldrinPoolSigner: ammPoolSigner,
        aldrinProgramToll: toll,
        aldrinProgramTollWallet,
        aldrinAmmProgram: amm.programId,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .remainingAccounts(vaults)
      .signers([user, ammId, adminTicketId, stakingId])
      .rpc();

    return [
      new AmmPool(ammId.publicKey, tollAuthority),
      new Staking(stakingId.publicKey),
    ];
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