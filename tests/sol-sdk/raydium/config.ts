import {
  ENDPOINT as _ENDPOINT,
  DEVNET_PROGRAM_ID,
  TxVersion,
  // LOOKUP_TABLE_CACHE
} from "@raydium-io/raydium-sdk";
import { PublicKey } from "@solana/web3.js";

export const PROGRAMIDS = DEVNET_PROGRAM_ID;

export const ENDPOINT = _ENDPOINT;

export const ATA_PROGRAM_ID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

export const makeTxVersion = TxVersion.V0; // LEGACY

//export const addLookupTableInfo = LOOKUP_TABLE_CACHE // only mainnet. other = undefined
export const addLookupTableInfo = undefined;
