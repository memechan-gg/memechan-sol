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
import { MemechanSol } from "../target/types/memechan_sol";
import { NATIVE_MINT, mintTo } from "@solana/spl-token";
import { config } from "dotenv";

import * as bigintBuffer from "bigint-buffer";
import { CHAN_TOKEN_INFO } from "./sol-sdk/config/config";

export const conf = config();

export const provider = AnchorProvider.local();
setProvider(provider);
export const payer = (provider.wallet as NodeWallet).payer;

const payerSecretKey = JSON.parse(process.env.ADMIN_PRIV_KEY ?? "");
export const adminSigner = Keypair.fromSecretKey(
  Uint8Array.from(payerSecretKey)
);
const mintSecretKey = JSON.parse(process.env.TEST_MINT_PRIV_KEY ?? "");
export const mintKeypair = Keypair.fromSecretKey(
  Uint8Array.from(mintSecretKey)
);

export const admin = adminSigner.publicKey;
console.log(`admkey ${adminSigner.publicKey.toBase58()}`);

export const QUOTE_MINT = new PublicKey(
  "So11111111111111111111111111111111111111112"
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
  if (QUOTE_MINT.equals(NATIVE_MINT)) {
    await airdrop(to, amount);
    return;
  }
  await mintTo(
    provider.connection,
    adminSigner,
    QUOTE_MINT,
    to,
    adminSigner,
    amount
  );
}

export async function mintChan(
  to: PublicKey,
  amount: number = 1_000_000_000_000_000
) {
  await mintTo(
    provider.connection,
    adminSigner,
    new PublicKey(CHAN_TOKEN_INFO.address),
    to,
    adminSigner,
    amount
  );
}

export async function sleep(ms: number) {
  await new Promise((r) => setTimeout(r, ms));
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
