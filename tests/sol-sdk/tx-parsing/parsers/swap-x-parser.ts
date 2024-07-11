import { ParsedTransactionWithMeta, PublicKey } from "@solana/web3.js";
import { MemechanClient } from "../../MemechanClient";

export type SwapXInstructionParsed = {
  sender: PublicKey;
  poolAddr: PublicKey;
  ticketAddr: PublicKey;
  baseAmtSwapped: number;
  quoteAmtReceived: number;
  poolQuoteVault: number;
  type: "swap_x";
};

export async function ParseSwapXInstruction(
  tx: ParsedTransactionWithMeta,
  index: number,
  client: MemechanClient,
): Promise<SwapXInstructionParsed | undefined> {
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

  const ticketAddr = ix.accounts[1];
  const ticket = await client.memechanProgram.account.memeTicket.fetchNullable(ticketAddr);

  if (!ticket) {
    throw new Error(`[ParseSwapXInstruction] No ticket found with ticket address ${ticketAddr}`);
  }

  const poolPrevPos = Number(preBalances[0].uiTokenAmount.amount);
  const poolQuoteVault = Number(postBalances[0].uiTokenAmount.amount);
  const baseAmtSwapped = poolQuoteVault - poolPrevPos;
  const quoteAmtReceived = ticket?.amount.toNumber();

  const swxParsed: SwapXInstructionParsed = {
    poolAddr,
    ticketAddr,
    quoteAmtReceived,
    poolQuoteVault,
    baseAmtSwapped,
    sender: tx.transaction.message.accountKeys[0].pubkey, // In the `Message` structure, the first account is always the fee-payer
    type: "swap_x",
  };

  return swxParsed;
}
