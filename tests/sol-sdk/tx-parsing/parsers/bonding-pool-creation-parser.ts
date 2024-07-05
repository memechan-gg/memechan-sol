import { ParsedTransactionWithMeta, PublicKey } from "@solana/web3.js";
import { IdlAccounts } from "@coral-xyz/anchor";
import { MemechanClient } from "../../MemechanClient";
import { MemechanSol } from "../../schema/types/memechan_sol";
import { Mint, getMint } from "@solana/spl-token";

export type Pool = IdlAccounts<MemechanSol>["boundPool"];

export type NewBPInstructionParsed = {
  sender: PublicKey;
  poolAddr: PublicKey;
  pool: Pool;
  newMint: Mint,
  type: "new_pool";
};

export async function ParseNewBPInstruction(
  tx: ParsedTransactionWithMeta,
  index: number,
  client: MemechanClient,
): Promise<NewBPInstructionParsed | undefined> {
  let poolAddr = PublicKey.default;
  const ix = tx.transaction.message.instructions[index];

  if ("accounts" in ix && ix.accounts.length > 2) { //to ensure we also have a mint, which has index of 2
    poolAddr = ix.accounts[1];
  } else {
    return undefined;
  }

  const pool = await client.memechanProgram.account.boundPool.fetchNullable(poolAddr);

  if (pool === null) {
    return undefined;
  }
  
  const newMintAddr = ix.accounts[2];
  const newMint = await getMint(client.connection, newMintAddr);

  const bpParsed: NewBPInstructionParsed = {
    poolAddr,
    pool,
    newMint,
    sender: tx.transaction.message.accountKeys[0].pubkey, // In the `Message` structure, the first account is always the fee-payer
    type: "new_pool",
  };

  return bpParsed;
}
