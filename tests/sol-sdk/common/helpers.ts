// import { Program, AnchorProvider, setProvider, BN, workspace } from "@coral-xyz/anchor";
// import { Connection, PublicKey } from "@solana/web3.js";
// import { MemechanSol } from "./types/memechan_sol";
// import { NATIVE_MINT, createWrappedNativeAccount } from "@solana/spl-token";
// import { Amm } from "./types/amm";

import { Connection, PublicKey } from "@solana/web3.js";

// export const memechan = workspace.MemechanSol as Program<MemechanSol>;
// export const amm = workspace.Amm as Program<Amm>;

// export const admin = new PublicKey("8vBA2MzaQdt3UWimSkx1J4m2zMgp8A2iwtRKzXVurXP2");

// export const solMint = NATIVE_MINT;

// export async function errLogs(job: Promise<unknown>): Promise<string> {
//   try {
//     await job;
//   } catch (error) {
//     if (!Array.isArray(error.logs)) {
//       console.log("No logs on the error:", error);
//       throw new Error(`No logs on the error object`);
//     }

//     return String(error.logs);
//   }

//   throw new Error("Expected promise to fail");
// }

export async function airdrop(connection: Connection, to: PublicKey, amount: number = 100_000_000_000) {
  await connection.confirmTransaction(await connection.requestAirdrop(to, amount), "confirmed");
}

export async function sleep(ms: number) {
  await new Promise((r) => setTimeout(r, ms));
}

// // export async function assertApproxCurrentSlot(
// //   connection: Connection,
// //   input: { slot: BN },
// //   delta: number = 2
// // ) {
// //   expect(input.slot.toNumber()).to.be.approximately(
// //     await getCurrentSlot(connection),
// //     delta
// //   );
// // }

// export function getCurrentSlot(connection: Connection): Promise<number> {
//   return connection.getSlot();
// }

export function findProgramAddress(seeds: Array<Buffer | Uint8Array>, programId: PublicKey) {
  const [publicKey, nonce] = PublicKey.findProgramAddressSync(seeds, programId);
  return { publicKey, nonce };
}
