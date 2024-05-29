import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeAccountInstruction,
} from "@solana/spl-token";
import {
  Commitment,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

export async function getCreateTokenAccountInstructions(
  connection: Connection,
  payer: PublicKey,
  mint: PublicKey,
  owner: PublicKey,
  keypair: Keypair,
  commitment?: Commitment,
  programId = TOKEN_PROGRAM_ID
): Promise<TransactionInstruction[]> {
  // Warning: That's an average space for the new account creation. It's used instead of `getMint()` method, because
  // it's not possible to call `getMint()` for not created token (e.g. for not created meme in `new()` method).
  const space = 165;
  const lamports = await connection.getMinimumBalanceForRentExemption(space);

  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer,
      newAccountPubkey: keypair.publicKey,
      space,
      lamports,
      programId,
    }),
    createInitializeAccountInstruction(
      keypair.publicKey,
      mint,
      owner,
      programId
    )
  );

  return transaction.instructions;
}

export async function getCreateAssociatedTokenAccountInstructions(
  addr: PublicKey,
  payer: PublicKey,
  mint: PublicKey,
  owner: PublicKey
): Promise<TransactionInstruction[]> {
  const transaction = new Transaction().add(
    createAssociatedTokenAccountIdempotentInstruction(payer, addr, owner, mint)
  );

  return transaction.instructions;
}
