import { Connection, PublicKey, Transaction } from "@solana/web3.js";
import {
  getAccount,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

export interface EnsureAssociatedTokenAccountWithIXArgs {
  connection: Connection;
  payer: PublicKey;
  owner: PublicKey;
  mint: PublicKey;
  transaction: Transaction;
}

export async function ensureAssociatedTokenAccountWithIX(
  args: EnsureAssociatedTokenAccountWithIXArgs
): Promise<PublicKey> {
  const { connection, payer, owner, mint, transaction } = args;
  const associatedTokenAddress = getAssociatedTokenAddressSync(
    mint,
    owner,
    true,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  try {
    await getAccount(connection, associatedTokenAddress);
  } catch (error) {
    const createAssociatedTokenAccountIX =
      createAssociatedTokenAccountInstruction(
        payer,
        associatedTokenAddress,
        owner,
        mint,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );
    transaction.add(createAssociatedTokenAccountIX);
  }

  return associatedTokenAddress;
}
