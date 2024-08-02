import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { memechan } from "./config/config";

export class UserStats {
  public static async CheckCreateStats(
    user: Keypair,
    pool: PublicKey,
    referral: PublicKey
  ) {
    const userStats = this.GetUserStatsPDA(user.publicKey);

    await memechan.methods
      .newUserStatsIdempotent()
      .accounts({
        userStats,
        pool,
        sender: user.publicKey,
        referral,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc({ skipPreflight: true });

    return userStats;
  }
  public static GetUserStatsPDA(user: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("ustats"), user.toBytes()],
      memechan.programId
    )[0];
  }
}
