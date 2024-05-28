import { LOOKUP_TABLE_CACHE, TxVersion } from "@raydium-io/raydium-sdk";
import { PublicKey } from "@solana/web3.js";

export const ATA_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
);

export const makeTxVersion = TxVersion.LEGACY; // LEGACY
export const addLookupTableInfo = LOOKUP_TABLE_CACHE; // only mainnet. other = undefined
// export const addLookupTableInfo = undefined;
