import { Program } from "@coral-xyz/anchor";
import { IDL } from "./raydium_cp_swap";
import { PublicKey } from "@solana/web3.js";

export const raydiumProgram = new Program(
  IDL,
  new PublicKey("CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW")
);
