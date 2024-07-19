import {
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { MAX_TRANSACTION_SIZE } from "../config/config";
import { getTxSize } from "../util/get-tx-size";

/**
 * Splits an array of instructions into multiple transactions, each with a size
 * less than or equal to the maximum allowed size.
 *
 * This method recursively divides the instruction array into two halves until
 * each transaction is within the allowed size limit. It aims to create the
 * smallest number of transactions possible, with each transaction being as
 * large as possible without exceeding the size limit.
 *
 * @param {TransactionInstruction[]} instructions - The array of instructions to split into transactions.
 * @param {PublicKey} feePayer - The account that will pay fees.
 * @returns {Transaction[]} An array of transactions, each with a size less than or equal to the maximum allowed size.
 */
export function getOptimizedTransactions(
  instructions: TransactionInstruction[],
  feePayer: PublicKey
): Transaction[] {
  /**
   * Recursively splits the instructions array into valid transactions.
   *
   * @param {TransactionInstruction[]} inst - The current array of instructions to process.
   * @returns {Transaction[]} An array of valid transactions.
   */
  function splitAndCreate(inst: TransactionInstruction[]): Transaction[] {
    if (inst.length === 0) {
      return [];
    }

    const transaction = new Transaction();
    transaction.add(...inst);

    if (getTxSize(transaction, feePayer) <= MAX_TRANSACTION_SIZE) {
      return [transaction];
    }

    const mid = Math.floor(inst.length / 2);
    const firstHalf = inst.slice(0, mid);
    const secondHalf = inst.slice(mid);

    return [...splitAndCreate(firstHalf), ...splitAndCreate(secondHalf)];
  }

  return splitAndCreate(instructions);
}
