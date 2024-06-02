import { PublicKey, Keypair } from "@solana/web3.js";
import { payer, provider } from "./helpers";
import { getAccount } from "@solana/spl-token";
import BN from "bn.js";
import { getAmmConfigAddress, swap_base_input } from "./utils";

export class AmmPool {
  swap(
    user: Keypair,
    solWallet: PublicKey,
    memeWallet: PublicKey,
    arg3: number,
    arg4: number
  ) {
    const token0 = ;
    const tx = await swap_base_input(raydiumProgram, payer, new PublicKey(""), );
  }
  public constructor(public id: PublicKey) {
    //
  }
}
