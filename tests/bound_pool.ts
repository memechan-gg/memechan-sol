import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  QUOTE_MINT,
  admin,
  adminSigner,
  airdrop,
  findProgramAddress,
  memechan,
  payer,
  provider,
} from "./helpers";
import BN from "bn.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createMint,
} from "@solana/spl-token";
import { DUMMY_TOKEN_METADATA, client } from "./common";
import { Token, publicKey } from "@raydium-io/raydium-sdk";
import { BoundPoolClient } from "./sol-sdk/bound-pool/BoundPool";
import { MemeTicket } from "./ticket";
import { AmmPool } from "./pool";
import { Staking } from "./staking";
import { IdlAccounts } from "@coral-xyz/anchor";
import { MemechanSol } from "../target/types/memechan_sol";

export const RAYDIUM_PROGRAM_ID = new PublicKey(
  "HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8"
);
export const OPENBOOK_ID = new PublicKey(
  "EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj"
);
export const MEMECHAN_MEME_TOKEN_DECIMALS = 6;
export const FEE_DESTINATION_ID = new PublicKey(
  "3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR"
);

export type BoundPool = IdlAccounts<MemechanSol>["boundPool"];

export interface SwapYArgs {
  user?: Keypair;
  memeTokensOut: BN;
  solAmountIn: BN;
}

export class BoundPoolWrapper {
  public async fetch(): Promise<BoundPool> {
    return memechan.account.boundPool.fetch(this.bpClient.id);
  }
  public async go_live(arg0: {}): Promise<[AmmPool, Staking]> {
    throw new Error("Method not implemented.");
  }
  public async swap_y(swapYArgs: SwapYArgs): Promise<MemeTicket> {
    throw new Error("Method not implemented.");
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
