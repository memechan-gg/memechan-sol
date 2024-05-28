import { ProgramId, TOKEN_PROGRAM_ID, Token } from "@raydium-io/raydium-sdk";
import { PublicKey } from "@solana/web3.js";
import BigNumber from "bignumber.js";
import {
  ENDPOINT as _ENDPOINT,
  DEVNET_PROGRAM_ID,
  LOOKUP_TABLE_CACHE,
  MAINNET_PROGRAM_ID,
  TxVersion,
} from "@raydium-io/raydium-sdk";

import * as dotenv from "dotenv";

dotenv.config(); // Load environment variables from .env file

export function raydiumPrograms(): ProgramId {
  if (process.env.DEVNET === "true") {
    return DEVNET_PROGRAM_ID;
  } else {
    return MAINNET_PROGRAM_ID;
  }
}

export function openbookPubkey(): PublicKey {
  if (process.env.DEVNET === "true") {
    return DEVNET_PROGRAM_ID.OPENBOOK_MARKET;
  } else {
    return MAINNET_PROGRAM_ID.OPENBOOK_MARKET;
  }
}

export function raydiumFeeVault(): PublicKey {
  if (process.env.DEVNET === "true") {
    return new PublicKey("3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR");
  } else {
    return new PublicKey("7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5");
  }
}

export function raydiumPubkey(): PublicKey {
  if (process.env.DEVNET === "true") {
    return DEVNET_PROGRAM_ID.AmmV4;
  } else {
    return MAINNET_PROGRAM_ID.AmmV4;
  }
}

export const RAYDIUM_PROTOCOL_FEE = 1_000_000_000; // 1 SOL on Devnet. TODO on Mainnet, it's only 0.4 SOL
export const TRANSFER_FEE = 60_000_000;

export const SLERF_MINT = new PublicKey(
  "7BgBvyjrZX1YKz4oh9mjb8ZScatkkwb8DzFx7LoiVkM3"
);

export const MEMECHAN_QUOTE_TOKEN_DECIMALS = 9;
export const MEMECHAN_MEME_TOKEN_DECIMALS = 6;

// Contract constants
export const MEME_TOKEN_DECIMALS = 1_000_000;
export const QUOTE_TOKEN_DECIMALS = 1_000_000_000;
export const MAX_TICKET_TOKENS = 800_000_000;
export const MAX_MEME_TOKENS = 1_000_000_000;

export const DEFAULT_PRICE_FACTOR = 2;
export const DEFAULT_MAX_M_LP = 200_000_000_000_000;
export const DEFAULT_MAX_M = 800_000_000_000_000;
export const DECIMALS_S = 1_000_000_000;
