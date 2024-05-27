import { ConfirmOptions, Connection, Signer, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";

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
