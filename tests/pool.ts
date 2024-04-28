import {
  PublicKey,
  Keypair,
} from "@solana/web3.js";
import { amm} from "./helpers";
import { BN } from "@project-serum/anchor";

export class AmmPool {
  private constructor(public id: Keypair, public admin: Keypair) {
    //
  }


  public async fetch() {
    return amm.account.pool.fetch(this.id.publicKey);
  }

  public static signerFrom(publicKey: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("signer"), publicKey.toBytes()],
      amm.programId
    )[0];
  }

  public signer(): PublicKey {
    return AmmPool.signerFrom(this.id.publicKey);
  }

  public signerPda(): PublicKey {
    return AmmPool.signerFrom(this.id.publicKey);
  }

  public async setSwapFee(permillion: number) {
    await amm.methods
      .setPoolSwapFee({
        permillion: new BN(permillion),
      })
      .accounts({ admin: this.admin.publicKey, pool: this.id.publicKey })
      .signers([this.admin])
      .rpc();
  }
}
