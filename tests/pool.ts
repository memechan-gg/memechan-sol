import { PublicKey, Keypair } from "@solana/web3.js";
import { getAmmConfigAddress, swap_base_input } from "./raydium/utils";
import { raydiumProgram } from "./raydium/raydium";
import BN from "bn.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { payer } from "./helpers";

export class AmmPool {
  async swap(user: Keypair, amountIn: number, amountOut: number) {
    const [configAddress, _] = await getAmmConfigAddress(
      0,
      raydiumProgram.programId
    );

    console.log(configAddress);
    // const token0 = staking.;
    console.log("s1");
    const tx = await swap_base_input(
      raydiumProgram,
      user,
      configAddress,
      this.quoteMint,
      TOKEN_PROGRAM_ID,
      this.memeMint,
      TOKEN_PROGRAM_ID,
      new BN(amountIn),
      new BN(amountOut)
    );
  }
  public constructor(
    public id: PublicKey,
    public memeMint: PublicKey,
    public quoteMint: PublicKey
  ) {
    //
  }
}
