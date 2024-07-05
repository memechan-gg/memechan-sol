import { Keypair, PublicKey } from "@solana/web3.js";
import { MemechanClient } from "../MemechanClient";
import BN from "bn.js";

export type CreateTargetConfigArgs = {
  client: MemechanClient;
  payer: Keypair;
  mint: PublicKey;
  targetAmount: BN;
}