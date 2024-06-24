import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  LUTSLOT,
  QUOTE_MINT,
  admin,
  adminSigner,
  airdrop,
  findProgramAddress,
  getLUTPDA,
  memechan,
  payer,
  provider,
  sleep,
} from "./helpers";
import BN from "bn.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
} from "@solana/spl-token";
import { DUMMY_TOKEN_METADATA, client } from "./common";
import { Token, publicKey } from "@raydium-io/raydium-sdk";
import { BoundPoolClient } from "./sol-sdk/bound-pool/BoundPool";
import { MemeTicketWrapper } from "./ticket";
import { AmmPool } from "./pool";
import { StakingWrapper } from "./staking";
import { IdlAccounts } from "@coral-xyz/anchor";
import { MemechanSol } from "../target/types/memechan_sol";
import { StakingPool } from "./sol-sdk/staking-pool/StakingPool";
import {
  getCreateAssociatedTokenAccountInstructions,
  getCreateTokenAccountInstructions,
} from "./sol-sdk/util/getCreateAccountInstruction";
import {
  CHAN_TOKEN_INFO,
  MEMECHAN_QUOTE_TOKEN,
  MEMECHAN_QUOTE_TOKEN_INFO,
} from "./sol-sdk/config/config";
import { associatedAddress } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { TokenInfo } from "@solana/spl-token-registry";
import AmmImpl from "@mercurial-finance/dynamic-amm-sdk";
import { ChanSwapWrapper } from "./chan_swap";

export const RAYDIUM_PROGRAM_ID = new PublicKey(
  "HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8"
);
export const OPENBOOK_ID = new PublicKey(
  "EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj"
);
export const MEMECHAN_MEME_TOKEN_DECIMALS = 6;
export const FEE_DESTINATION_ID = new PublicKey(
  "G11FKBRaAkHAKuLCgLM6K6NUc9rTjPAznRCjZifrTQe2"
);

export type BoundPoolType = IdlAccounts<MemechanSol>["boundPool"];

export interface SwapYArgs {
  user?: Keypair;
  memeTokensOut: BN;
  quoteTokensIn: BN;
  userQuoteAcc?: PublicKey;
  ticketNumber?: number;
}

export class BoundPoolWrapper {
  public async fetch(): Promise<BoundPoolType> {
    return memechan.account.boundPool.fetch(this.bpClient.id);
  }
  public async go_live(): Promise<[AmmPool, StakingWrapper]> {
    console.log("go_live");
    const res = await this.bpClient.initStakingPool({
      boundPoolInfo: this.bpClient.poolInfo,
      payer,
      user: payer,
      pool: this.bpClient.id,
    });

    const stakingFetched = await memechan.account.stakingPool.fetch(
      res.staking
    );

    const tokenInfoA: TokenInfo = {
      chainId: 0,
      address: stakingFetched.memeMint.toBase58(),
      name: "asdda",
      decimals: MEMECHAN_MEME_TOKEN_DECIMALS,
      symbol: "ads",
    };
    const tokenInfoB = MEMECHAN_QUOTE_TOKEN_INFO;
    const tokenInfoC = CHAN_TOKEN_INFO;

    const staking = await this.bpClient.initQuoteAmmPool({
      boundPoolInfo: this.bpClient.poolInfo,
      feeDestinationWalletAddress: FEE_DESTINATION_ID,
      memeVault: res.stakingMemeVault,
      quoteVault: res.stakingQuoteVault,
      payer,
      user: payer,
      tokenInfoA,
      tokenInfoB,
    });
    console.log("goLive2 END");
    console.log(staking.memeMint, staking.amm);

    const chanSwap = ChanSwapWrapper.chanSwapId();

    await this.bpClient.initChanAmmPool({
      boundPoolInfo: this.bpClient.poolInfo,
      feeDestinationWalletAddress: FEE_DESTINATION_ID,
      memeVault: res.stakingMemeVault,
      quoteVault: res.stakingQuoteVault,
      payer,
      user: payer,
      tokenInfoA,
      tokenInfoB: tokenInfoC,
      chanSwap,
    });
    console.log("goLive2 END");
    console.log(staking.memeMint, staking.amm);

    await sleep(500);
    const ammImpl = await AmmImpl.create(
      provider.connection,
      staking.amm,
      tokenInfoA,
      tokenInfoB
    );

    return [
      new AmmPool(staking.amm, staking.memeMint, QUOTE_MINT, ammImpl),
      new StakingWrapper(staking.id),
    ];
  }
  public async swap_y(args: SwapYArgs): Promise<MemeTicketWrapper> {
    const user = args.user ?? payer;
    const memeTokensOut = args.memeTokensOut;
    const quoteAmountIn = args.quoteTokensIn;

    const userQuoteAcc =
      args.userQuoteAcc ??
      (
        await getOrCreateAssociatedTokenAccount(
          client.connection,
          payer,
          QUOTE_MINT,
          user.publicKey
        )
      ).address;
    console.log(userQuoteAcc);
    await transfer(
      provider.connection,
      payer,
      associatedAddress({ mint: QUOTE_MINT, owner: payer.publicKey }),
      userQuoteAcc,
      payer,
      quoteAmountIn.toNumber()
    );
    const ticket = await this.bpClient.swapY({
      memeTokensOut,
      quoteAmountIn,
      payer,
      pool: this.bpClient.id,
      quoteMint: QUOTE_MINT,
      user,
      userQuoteAcc,
    });

    return new MemeTicketWrapper(ticket.id);
  }
  private constructor(public bpClient: BoundPoolClient) {
    //
  }

  public static async new(): Promise<BoundPoolWrapper> {
    const bpClient = await BoundPoolClient.new({
      admin: admin,
      client,
      payer,
      quoteToken: {
        programId: TOKEN_PROGRAM_ID,
        mint: QUOTE_MINT,
        equals: function (other: Token): boolean {
          throw new Error("Function not implemented.");
        },
        decimals: 9,
      },
      tokenMetadata: DUMMY_TOKEN_METADATA,
      lutAddr: getLUTPDA({
        authority: admin,
        recentSlot: LUTSLOT,
      }),
    });

    return new BoundPoolWrapper(bpClient);
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

  static getATAAddress(
    owner: PublicKey,
    mint: PublicKey,
    programId: PublicKey
  ) {
    return findProgramAddress(
      [owner.toBuffer(), programId.toBuffer(), mint.toBuffer()],
      new PublicKey(ASSOCIATED_TOKEN_PROGRAM_ID)
    );
  }

  static getAssociatedId({
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
    return publicKey;
  }

  public static findTargetConfigPda(
    quoteMintPubkey: PublicKey,
    memechanProgramId: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("config"), quoteMintPubkey.toBytes()],
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

  public static findSignerPda(
    publicKey: PublicKey,
    memechanProgramId: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("signer"), publicKey.toBytes()],
      memechanProgramId
    )[0];
  }
}
