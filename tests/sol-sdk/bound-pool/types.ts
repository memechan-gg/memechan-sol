import { BN } from "@coral-xyz/anchor";
import { Token } from "@raydium-io/raydium-sdk";
import { Keypair, PublicKey, Signer, Transaction } from "@solana/web3.js";
import { MemechanClient } from "../MemechanClient";
import { MemeTicket } from "../memeticket/MemeTicket";
import { TokenMetadata } from "../token/types";
import { BoundPool } from "../../bound_pool";

export interface SwapYArgs {
  payer: Signer;
  user: Keypair;
  pool: PublicKey;
  userSolAcc?: PublicKey;
  quoteAmountIn: BN;
  quoteMint: PublicKey;
  memeTokensOut: BN;
}

export type GetBuyMemeTransactionArgs = {
  user: PublicKey;
  inputTokenAccount?: PublicKey;
  inputAmount: string;
  minOutputAmount: string;
  slippagePercentage: number;
  transaction?: Transaction;
};

export type BuyMemeArgs = GetBuyMemeTransactionArgs & { signer: Keypair };

export type GetOutputAmountForBuyMeme = Omit<BuyMemeArgs, "minOutputAmount">;

export type GetBuyMemeTransactionOutput = {
  tx: Transaction;
  memeTicketKeypair: Keypair;
  inputTokenAccount: PublicKey;
};

export interface SwapXArgs {
  user: Keypair;
  //pool: PublicKey;
  //poolSignerPda: PublicKey;
  memeAmountIn: BN;
  minQuoteAmountOut: BN;
  userMemeTicket: MemeTicket;
  userQuoteAcc: PublicKey;
  quoteMint: PublicKey;
}

export type GetSellMemeTransactionArgs = Omit<
  SwapXArgs,
  "user" | "pool" | "poolSignerPda"
> & {
  user: { publicKey: PublicKey };
  transaction?: Transaction;
};

export interface GoLiveArgs {
  user: Keypair;
  payer: Signer;
  boundPoolInfo: BoundPool;
  memeVault: PublicKey;
  feeDestinationWalletAddress: string;
  quoteVault: PublicKey;
}

export type GetGoLiveTransactionArgs = GoLiveArgs & {
  transaction?: Transaction;
};

export interface InitStakingPoolArgs {
  pool?: PublicKey;
  user: Keypair;
  payer: Signer;
  boundPoolInfo: BoundPool;
}

export type GetInitStakingPoolTransactionArgs = Omit<
  InitStakingPoolArgs,
  "user"
> & {
  user: PublicKey;
  transaction?: Transaction;
};

export interface BoundPoolArgs {
  admin: PublicKey;
  payer: Signer;
  client: MemechanClient;
  quoteToken: Token;
  tokenMetadata: TokenMetadata;
}

export type GetCreateNewBondingPoolAndTokenTransactionArgs = Omit<
  BoundPoolArgs,
  "payer"
> & {
  payer: PublicKey;
  transaction?: Transaction;
  adminSolPublicKey?: PublicKey;
};

export interface InitStakingPoolResult {
  stakingMemeVault: PublicKey;
  stakingQuoteVault: PublicKey;
}
