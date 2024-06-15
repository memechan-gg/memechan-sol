import { PublicKey, Keypair } from "@solana/web3.js";

import BN from "bn.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { payer } from "./helpers";

export class AmmPool {
  async swap(user: Keypair, amountIn: number, amountOut: number) {}
  public constructor(
    public id: PublicKey,
    public memeMint: PublicKey,
    public quoteMint: PublicKey
  ) {
    //
  }
}
