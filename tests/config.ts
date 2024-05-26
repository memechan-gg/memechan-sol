import { TOKEN_PROGRAM_ID, Token } from "@raydium-io/raydium-sdk";
import { PublicKey } from "@solana/web3.js";
import BigNumber from "bignumber.js";

/**
 * The base URL for the backend API for fetching off-chain data.
 * This endpoint ideally should point to the environment-specific endpoint, but currently it's only prod one
 *
 * @constant {string}
 */
export const BE_URL =
  "https://h9crl8krnj.execute-api.us-east-1.amazonaws.com/prod";
export const BE_REGION = "us-east-1";

/**
 * The Memechan program ID on Solana.
 * This is the entry point of the Memechan smart contract.
 *
 * Be aware, that the same program address should be specified in IDL (`src/idl/memechan_sol.json`)
 *
 * @constant {string}
 */
export const MEMECHAN_PROGRAM_ID =
  "kKRWbmqpGqPzkAb9aLmhCFcJsr3oBkqCcZYK21dWYRz";
export const RAYDIUM_PROTOCOL_FEE = 1_000_000_000; // 1 SOL on Devnet. TODO on Mainnet, it's only 0.4 SOL
export const TRANSFER_FEE = 60_000_000;
export const COMPUTE_UNIT_PRICE = 100_000; // priority fee

export const SLERF_MINT = new PublicKey(
  "7BgBvyjrZX1YKz4oh9mjb8ZScatkkwb8DzFx7LoiVkM3"
);

export const MEMECHAN_QUOTE_MINT = new PublicKey(
  "HX2pp5za2aBkrA5X5iTioZXcrpWb2q9DiaeWPW3qKMaw"
);
export const MEMECHAN_TARGET_CONFIG = new PublicKey(
  "H3vkkiCoXWESVK2FGUa561XtC41g73sRgx4h3tQJNAov"
);
export const MEMECHAN_QUOTE_TOKEN_DECIMALS = 9; // current devnet quote token decimals
export const MEMECHAN_QUOTE_TOKEN: Token = new Token(
  TOKEN_PROGRAM_ID,
  MEMECHAN_QUOTE_MINT,
  MEMECHAN_QUOTE_TOKEN_DECIMALS,
  "SLERF",
  "SLERF"
);
export const MEMECHAN_MEME_TOKEN_DECIMALS = 6;

// Contract constants
export const MEME_TOKEN_DECIMALS = 1_000_000;
export const WSOL_DECIMALS = 1_000_000_000;
export const MAX_TICKET_TOKENS = 800_000_000;
export const MAX_MEME_TOKENS = 1_000_000_000;

export const DEFAULT_PRICE_FACTOR = 2;
export const DEFAULT_MAX_M_LP = 200_000_000_000_000;
export const DEFAULT_MAX_M = 800_000_000_000_000;

export const DECIMALS_S = 1_000_000_000;

export const MAX_TRANSACTION_SIZE = 1232;
export const ADMIN_PUB_KEY = new PublicKey(
  "8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV"
);
export const FULL_MEME_AMOUNT_CONVERTED = new BigNumber(DEFAULT_MAX_M_LP)
  .plus(DEFAULT_MAX_M)
  .div(10 ** MEMECHAN_MEME_TOKEN_DECIMALS)
  .toString();
