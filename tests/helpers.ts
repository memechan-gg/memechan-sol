import {
  Program,
  workspace,
  AnchorProvider,
  setProvider,
  BN,
} from "@coral-xyz/anchor";
import {
  PublicKey,
  Keypair,
  Connection,
  Transaction,
  Signer,
  ConfirmOptions,
  sendAndConfirmTransaction,
  AddressLookupTableProgram,
} from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { expect } from "chai";
import { MemechanSol } from "../target/types/memechan_sol";
import { mintTo } from "@solana/spl-token";
import { config } from "dotenv";

import * as bigintBuffer from "bigint-buffer";

export const conf = config();

export const provider = AnchorProvider.local();
setProvider(provider);
export const payer = (provider.wallet as NodeWallet).payer;

export const memechan = workspace.MemechanSol as Program<MemechanSol>;

export const admin = new PublicKey(
  "8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV"
);

const payerSecretKey = JSON.parse(process.env.ADMIN_PRIV_KEY ?? "");
export const adminSigner = Keypair.fromSecretKey(
  Uint8Array.from(payerSecretKey)
);

export const QUOTE_MINT = new PublicKey(
  "HX2pp5za2aBkrA5X5iTioZXcrpWb2q9DiaeWPW3qKMaw"
);

export const LUTSLOT: number = 2;
export const LUT2SLOT: number = 20;

export function getSendAndConfirmTransactionMethod({
  connection,
  transaction,
  signers,
  options = {
    commitment: "confirmed",
    skipPreflight: true,
  },
}: {
  connection: Connection;
  transaction: Transaction;
  signers: Signer[];
  options?: ConfirmOptions;
}): () => Promise<void> {
  return async () => {
    await sendAndConfirmTransaction(connection, transaction, signers, options);
  };
}

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

export async function airdrop_tokens(
  to: PublicKey,
  amount: number = 1 ** 6 * 10 ** 9
) {
  return await mintQuote(to, amount);
}

export function findProgramAddress(
  seeds: Array<Buffer | Uint8Array>,
  programId: PublicKey
) {
  const [publicKey, nonce] = PublicKey.findProgramAddressSync(seeds, programId);
  return { publicKey, nonce };
}

export async function mintQuote(
  to: PublicKey,
  amount: number = 1_000_000_000_000_000
) {
  await mintTo(
    provider.connection,
    adminSigner,
    QUOTE_MINT,
    to,
    adminSigner,
    amount
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

export function getLUTPDA(params: {
  recentSlot: number;
  authority: PublicKey;
}) {
  const [lookupTableAddress, bumpSeed] = PublicKey.findProgramAddressSync(
    [
      params.authority.toBuffer(),
      bigintBuffer.toBufferLE(BigInt(params.recentSlot), 8),
    ],
    AddressLookupTableProgram.programId
  );
  return lookupTableAddress;
}
