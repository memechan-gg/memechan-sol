import { TOKEN_PROGRAM_ID, Token } from "@raydium-io/raydium-sdk";
import { PublicKey } from "@solana/web3.js";
import { memechan } from "../../helpers";

/**
 * The base URL for the backend API for fetching off-chain data.
 * This endpoint ideally should point to the environment-specific endpoint, but currently it's only prod one
 *
 * @constant {string}
 */
export const BE_URL =
  "https://7mgmqkuj18.execute-api.us-east-1.amazonaws.com/prod";
export const BE_REGION = "us-east-1";

/**
 * The Memechan program ID on Solana.
 * This is the entry point of the Memechan smart contract.
 *
 * Be aware, that the same program address should be specified in IDL (`src/idl/memechan_sol.json`)
 *
 * @constant {string}
 */
export const SLERF_MINT = new PublicKey(
  "7BgBvyjrZX1YKz4oh9mjb8ZScatkkwb8DzFx7LoiVkM3"
);

export const MEMECHAN_QUOTE_MINT = new PublicKey(
  "HX2pp5za2aBkrA5X5iTioZXcrpWb2q9DiaeWPW3qKMaw"
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
export const MAX_TICKET_TOKENS = 900_000_000;
export const MAX_MEME_TOKENS = 1_125_000_000;

export const DEFAULT_PRICE_FACTOR = 2;
export const DEFAULT_MAX_M_LP = 200_000_000_000_000;
export const DEFAULT_MAX_M = 900_000_000_000_000;
export const DEFAULT_MAX_S = 300;

export const DECIMALS_ALPHA = 1_000_000; // consider increase
export const DECIMALS_BETA = 1_000_000; // consider increase

export const MAX_TRANSACTION_SIZE = 1232;

export const ADMIN_PUB_KEY = new PublicKey(
  "8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV"
);

export const MEMECHAN_PROGRAM_ID = memechan.programId;
