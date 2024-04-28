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
import { airdrop, memechan, payer, provider, admin, solMint, amm, sleep } from "./helpers";
import { createProgramToll, programTollAddress } from "./amm";
import { BN } from "@project-serum/anchor";
import { AmmPool } from "./pool";
import { Staking } from "./staking";

export interface SwapYArgs { 
  user: Keypair;
  pool: PublicKey;
  poolSignerPda: PublicKey;
  userSolAcc: PublicKey;
  solAmountIn: BN;
  memeTokensOut: BN;
  vaultsAndWallets: AccountMeta[];
}

export interface SwapXArgs { 
  user: Keypair;
  pool: PublicKey;
  poolSignerPda: PublicKey;
  memeAmountIn: BN;
  solTokensOut: BN;
  userMemeTicket: PublicKey;
  userSolAcc: PublicKey;
  vaultsAndWallets: AccountMeta[];
}

export interface GoLiveArgs {
  pool: PublicKey;
  user: Keypair;
  poolSignerPda: PublicKey;
}


export class BoundPool {
  private constructor(public id: PublicKey, public admin: Keypair, public solVault: PublicKey) {
    //
  }

  public static async new(): Promise<BoundPool> {
    provider.opts.skipPreflight = true
    const id = Keypair.generate();

    

    // let [id, _] = PublicKey.findProgramAddressSync(
    //   [Buffer.from("signer"), id.toBuffer()],
    //   memechan.programId
    // );

    const signer = Keypair.generate();
    await airdrop(signer.publicKey);

    // let tollAuthority = payer.publicKey;
    // try {
    //   const info = await memechan.account.programToll.fetch(toll);
    //   tollAuthority = info.authority;
    // } catch {
    //   await createProgramToll(tollAuthority);
    // }

    const poolSigner = BoundPool.signerFrom(id.publicKey);

    const adminAuthority = admin;

    const memeMint = await createMint(
      provider.connection,
      payer,
      poolSigner,
      null,
      6
    );
    
    const adminSolVault = (await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      solMint,
      adminAuthority
    )).address;
    
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
   
    await memechan.methods
      .new()
      .accounts({
        adminSolVault: adminSolVault,
        launchVault: launchVault, 
        solVault: poolSolVault,
        memeMint: memeMint,
        pool: id.publicKey,
        poolSigner: poolSigner,
        sender: signer.publicKey,
        solMint: solMint,
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

  public static async airdropLiquidityTokens(
    mint: PublicKey,
    wallet: PublicKey,
    authority: Signer,
    amount: number = 1_000_000
  ) {
    return mintTo(provider.connection, payer, mint, wallet, authority, amount);
  }

  public async swap_y(
    input: Partial<SwapYArgs>
  ): Promise<PublicKey> {
    const user = input.user ?? Keypair.generate();
    await airdrop(user.publicKey);

    const id = Keypair.generate();

    const pool = input.pool ?? this.id;
    const poolSignerPda = input.poolSignerPda ?? this.signerPda();
    const sol_in = input.solAmountIn ?? 1 * 1e9;
    const meme_out = input.memeTokensOut ?? 1;

    const userSolAddr = getAssociatedTokenAddressSync(
      NATIVE_MINT,
      user.publicKey
    );
    const userSolAcc = input.userSolAcc ?? await createWrappedNativeAccount(
      provider.connection,
      payer,
      user.publicKey,
      500 * 10e9
    );

    await memechan.methods
      .swapY(new BN(sol_in), new BN(meme_out))
      .accounts({
        memeTicket: id.publicKey,
        owner: user.publicKey,
        pool: pool,
        poolSignerPda: poolSignerPda,
        solVault: this.solVault,
        userSol: userSolAcc,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user, id])
      .rpc();

      return id.publicKey
  }

  public async swap_x(
    input: Partial<SwapXArgs>
  ): Promise<void> {
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
        memeTicket,
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
  ): Promise<void> {
    const user = input.user ?? Keypair.generate();
    await airdrop(user.publicKey);
    const ammId = Keypair.generate();

    const pool = input.pool ?? this.id;
    const poolSigner = input.poolSignerPda ?? this.signerPda();
    const ammPoolSigner = AmmPool.signerFrom(ammId.publicKey);
    
    const adminTicketId = Keypair.generate();

    const stakingId = Keypair.generate();
    const stakingSigner = Staking.signerFrom(stakingId.publicKey);

    const poolInfo = await memechan.account.boundPool.fetch(pool);

    const lpMint = await createMint(
      provider.connection,
      user,
      ammPoolSigner,
      null,
      9
    );

    console.log("2");
    const lpTokenWalletId = Keypair.generate();
    const lpTokenWallet = await createAccount(
      provider.connection,
      user,
      lpMint,
      poolSigner,
      lpTokenWalletId
    );

    console.log("3");
    const toll = await programTollAddress();
    console.log("3.1")
    let tollAuthority = payer.publicKey;
    try {
      const info = await amm.account.programToll.fetch(toll);
      tollAuthority = info.authority;
      console.log("3.1.1")
    } catch {
      console.log("3.1.2")
      await createProgramToll(tollAuthority);
    }

    console.log("4");
    const aldrinProgramTollWalletId = Keypair.generate()
    const aldrinProgramTollWallet = await createAccount(
      provider.connection,
      payer,
      lpMint,
      tollAuthority,
      aldrinProgramTollWalletId
    );

    console.log("5");
    const stakingMemeVaultId = Keypair.generate()
    const stakingMemeVault = await createAccount(
      provider.connection,
      payer,
      poolInfo.memeMint,
      stakingSigner,
      stakingMemeVaultId
    );

    console.log("6");
    const nkey = Keypair.generate();
    const userSolAcc = await createWrappedNativeAccount(
      provider.connection,
      payer,
      user.publicKey,
      1e9,
      nkey
    );
    console.log("7");
    await closeAccount(
      provider.connection,
      payer,
      userSolAcc,
      stakingSigner,
      user
    )

    await sleep(1000);

    console.log("8");
    const vaults = await Promise.all(
      [poolInfo.memeMint, poolInfo.solReserve.mint].map(async (mint) => {
        
        const kp = Keypair.generate();
        await createAccount(provider.connection, payer, mint, ammPoolSigner, kp);
        return {
          isSigner: false,
          isWritable: true,
          pubkey: kp.publicKey,
        };
      })
    );

    
    console.log("9");
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
        stakingMemeVault,
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

