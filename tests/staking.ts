import {
  NATIVE_MINT,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createAccount,
  getAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import {
  PublicKey,
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { QUOTE_MINT, payer, provider } from "./helpers";
import { AmmPool } from "./pool";
import { Address, BN, IdlAccounts } from "@coral-xyz/anchor";
import { MemeTicketWrapper } from "./ticket";
import { StakingPool } from "./sol-sdk/staking-pool/StakingPool";
import {
  createProgram,
  deriveLockEscrowPda,
  deriveMintMetadata,
  derivePoolAddress,
  getAssociatedTokenAccount,
} from "@mercurial-finance/dynamic-amm-sdk/dist/cjs/src/amm/utils";
import { SEEDS } from "@mercurial-finance/dynamic-amm-sdk/dist/cjs/src/amm/constants";
import VaultImpl, { getVaultPdas } from "@mercurial-finance/vault-sdk";
import { TokenInfo } from "@solana/spl-token-registry";
import { MEMECHAN_MEME_TOKEN_DECIMALS } from "./bound_pool";
import {
  CHAN_TOKEN_INFO,
  MEMECHAN_QUOTE_TOKEN,
  memechan,
} from "./sol-sdk/config/config";
import { MemechanSol } from "../target/types/memechan_sol";
import { LP_FEE_VAULT_OWNER } from "./common";

export type Staking = IdlAccounts<MemechanSol>["stakingPool"];

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

  public async add_fees(quoteAmmPool: AmmPool, chanAmmPool: AmmPool) {
    const staking = await memechan.account.stakingPool.fetch(this.id);

    const memeInfo: TokenInfo = {
      chainId: 0,
      address: staking.memeMint.toBase58(),
      name: "",
      decimals: MEMECHAN_MEME_TOKEN_DECIMALS,
      symbol: "",
    };
    const quoteInfo: TokenInfo = {
      chainId: 0,
      address: MEMECHAN_QUOTE_TOKEN.mint.toBase58(),
      name: MEMECHAN_QUOTE_TOKEN.name,
      decimals: MEMECHAN_QUOTE_TOKEN.decimals,
      symbol: MEMECHAN_QUOTE_TOKEN.symbol,
    };
    const chanInfo = CHAN_TOKEN_INFO;

    await this.add_fees_one_pool(quoteAmmPool, staking, memeInfo, quoteInfo);
    await this.add_fees_one_pool(chanAmmPool, staking, memeInfo, chanInfo);
  }

  public async add_fees_one_pool(
    ammPool: AmmPool,
    staking: Staking,
    tokenInfoA: TokenInfo,
    tokenInfoB: TokenInfo
  ) {
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
    let preInstructions: Array<TransactionInstruction> = [];

    if (!aVaultAccount) {
      const createVaultAIx =
        await VaultImpl.createPermissionlessVaultInstruction(
          provider.connection,
          payer.publicKey,
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
          payer.publicKey,
          tokenInfoB
        );
      createVaultBIx && preInstructions.push(createVaultBIx);
    } else {
      bVaultLpMint = bVaultAccount.lpMint; // Old vault doesn't have lp mint pda
    }

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

    const stakingSigner = this.signerPda();

    const [lockEscrowPK] = deriveLockEscrowPda(
      poolPubkey,
      stakingSigner,
      ammProgram.programId
    );

    preInstructions = [];

    const payerPoolLp = await getAssociatedTokenAccount(lpMint, stakingSigner);

    const escrowAta = await getAssociatedTokenAccount(lpMint, lockEscrowPK);

    const memeFeeVault = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        tokenAMint,
        LP_FEE_VAULT_OWNER,
        true
      )
    ).address;
    const quoteFeeVault = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        tokenBMint,
        LP_FEE_VAULT_OWNER,
        true
      )
    ).address;

    const quoteVault =
      tokenInfoB.address === CHAN_TOKEN_INFO.address
        ? staking.chanVault
        : staking.quoteVault;

    await memechan.methods
      .addFees()
      .accounts({
        memeFeeVault,
        quoteFeeVault,
        ammPool: ammPool.id,
        aTokenVault,
        aVault,
        aVaultLp,
        aVaultLpMint,
        bTokenVault,
        bVault,
        bVaultLp,
        bVaultLpMint,
        escrowVault: escrowAta,
        lockEscrow: lockEscrowPK,
        lpMint,
        memeMint: staking.memeMint,
        memeVault: staking.memeVault,
        quoteMint: tokenBMint,
        quoteVault,
        signer: payer.publicKey,
        sourceTokens: payerPoolLp,
        staking: this.id,
        stakingSignerPda: stakingSigner,
        ammProgram: ammProgram.programId,
        vaultProgram: vaultProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        memoProgram: MEMO_PROGRAM_ID,
      })
      .signers([payer])
      .rpc({ skipPreflight: true });
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
    const chanAcc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      new PublicKey(CHAN_TOKEN_INFO.address),
      user.publicKey
    );

    await memechan.methods
      .withdrawFees()
      .accounts({
        memeTicket: input.ticket.id,
        stakingSignerPda: this.signer(),
        memeVault: stakingInfo.memeVault,
        quoteVault: stakingInfo.quoteVault,
        chanVault: stakingInfo.chanVault,
        staking: this.id,
        userMeme: memeAcc,
        userQuote: quoteAcc.address,
        userChan: chanAcc.address,
        signer: user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    return [memeAcc, quoteAcc.address];
  }
}
