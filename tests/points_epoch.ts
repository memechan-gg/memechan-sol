import BN from "bn.js";
import {
  QUOTE_MINT,
  adminSigner,
  airdrop,
  mintChan,
  mintQuote,
  payer,
} from "./helpers";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { memechan } from "./sol-sdk/config/config";

export class PointsEpochWrapper {
  static pointsEpochPDA(): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("points_epoch")],
      memechan.programId
    )[0];
  }

  static async changeCreate(
    number: number,
    pointNum: number,
    pointsDenom: number
  ) {
    await memechan.methods
      .changePointsEpoch(new BN(number), new BN(pointNum), new BN(pointsDenom))
      .accounts({
        pointsEpoch: PointsEpochWrapper.pointsEpochPDA(),
        sender: adminSigner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([adminSigner])
      .rpc({ skipPreflight: true });
  }
}
