import { PublicKey, Keypair } from "@solana/web3.js";
import { provider } from "./helpers";
import { getAccount } from "@solana/spl-token";
import BN from "bn.js";

export class AmmPool {
  swap(
    user: Keypair,
    solWallet: PublicKey,
    memeWallet: PublicKey,
    arg3: number,
    arg4: number
  ) {
    throw new Error("Method not implemented.");
  }
  public constructor(public id: PublicKey) {
    //
  }
}
