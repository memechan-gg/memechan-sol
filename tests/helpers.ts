import { AnchorProvider, setProvider, BN } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { expect } from "chai";
import { Program, workspace } from "@project-serum/anchor";
import { MemechanSol } from "../target/types/memechan_sol";
import { NATIVE_MINT, createWrappedNativeAccount } from "@solana/spl-token";

export const provider = AnchorProvider.local();
setProvider(provider);
export const payer = (provider.wallet as NodeWallet).payer;

export const memechan = workspace.MemechanSol as Program<MemechanSol>;

export const admin = new PublicKey("8vBA2MzaQdt3UWimSkx1J4m2zMgp8A2iwtRKzXVurXP2");

export const solMint = NATIVE_MINT;

export async function errLogs(job: Promise<unknown>): Promise<string> {
  try {
    await job;
  } catch (error) {
    if (!Array.isArray(error.logs)) {
      console.log("No logs on the error:", error);
      throw new Error(`No logs on the error object`);
    }

    return String(error.logs);
  }

  throw new Error("Expected promise to fail");
}

export async function airdrop(to: PublicKey, amount: number = 100_000_000_000) {
  await provider.connection.confirmTransaction(
    await provider.connection.requestAirdrop(to, amount),
    "confirmed"
  );
}

export async function sleep(ms: number) {
  await new Promise((r) => setTimeout(r, ms));
}

export async function assertApproxCurrentSlot(
  input: { slot: BN },
  delta: number = 2
) {
  expect(input.slot.toNumber()).to.be.approximately(
    await getCurrentSlot(),
    delta
  );
}

export function getCurrentSlot(): Promise<number> {
  return provider.connection.getSlot();
}
