import { PublicKey, Keypair, sendAndConfirmTransaction } from "@solana/web3.js";

import BN from "bn.js";
import { payer, provider } from "./helpers";
import AmmImpl from "@mercurial-finance/dynamic-amm-sdk";

export class AmmPool {
  async swap(user: Keypair, amountIn: number, amountOut: number) {
    const swapTx = await this.ammImpl.swap(
      user.publicKey,
      this.quoteMint,
      new BN(amountIn),
      new BN(amountOut)
    );

    console.log(
      "swap tx",
      await sendAndConfirmTransaction(provider.connection, swapTx, [user])
    );
  }

  public constructor(
    public id: PublicKey,
    public memeMint: PublicKey,
    public quoteMint: PublicKey,
    public ammImpl: AmmImpl
  ) {}
}
