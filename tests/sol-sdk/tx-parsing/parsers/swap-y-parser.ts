import { ParsedTransactionWithMeta, PublicKey } from "@solana/web3.js";
import { IdlAccounts } from "@coral-xyz/anchor";
import { MemechanClient } from "../../MemechanClient";
import { MemechanSol } from "../../schema/types/memechan_sol";

export type MemeTicket = IdlAccounts<MemechanSol>["memeTicket"];

export type SwapYInstructionParsed = {
  sender: PublicKey;
  poolAddr: PublicKey;
  ticket: MemeTicket;
  quoteAmtSwapped: number;
  baseAmtReceived: number;
  poolQuoteVault: number;
  type: "swap_y";
};

export async function ParseSwapYInstruction(
  tx: ParsedTransactionWithMeta,
  index: number,
  client: MemechanClient,
): Promise<SwapYInstructionParsed | undefined> {
  const ix = tx.transaction.message.instructions[index];

  if (!("accounts" in ix)) {
    return undefined;
  }

  const poolAddr = ix.accounts[0];

  const preBalances = tx.meta?.preTokenBalances;
  const postBalances = tx.meta?.postTokenBalances;

  if (!preBalances || !postBalances) {
    return undefined;
  }

  const ticketAddr = ix.accounts[3];
  const ticket = await client.memechanProgram.account.memeTicket.fetchNullable(ticketAddr);

  if (!ticket) {
    throw new Error(`[ParseSwapYInstruction] No ticket found with ticket address ${ticketAddr}`);
  }

  const poolPrevPos = Number(preBalances[0].uiTokenAmount.amount);
  const poolQuoteVault = Number(postBalances[0].uiTokenAmount.amount);
  const quoteAmtSwapped = poolQuoteVault - poolPrevPos;
  const baseAmtReceived = ticket.amount.toNumber();

  const swyParsed: SwapYInstructionParsed = {
    poolAddr,
    ticket,
    baseAmtReceived,
    poolQuoteVault,
    quoteAmtSwapped,
    sender: tx.transaction.message.accountKeys[0].pubkey, // In the `Message` structure, the first account is always the fee-payer
    type: "swap_y",
  };

  return swyParsed;
}
