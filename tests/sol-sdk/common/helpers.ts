import { Connection, PublicKey } from "@solana/web3.js";

export async function airdrop(
  connection: Connection,
  to: PublicKey,
  amount: number = 100_000_000_000
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(to, amount),
    "confirmed"
  );
}

export async function sleep(ms: number) {
  await new Promise((r) => setTimeout(r, ms));
}

export function findProgramAddress(
  seeds: Array<Buffer | Uint8Array>,
  programId: PublicKey
) {
  const [publicKey, nonce] = PublicKey.findProgramAddressSync(seeds, programId);
  return { publicKey, nonce };
}
