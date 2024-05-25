import {
  buildSimpleTransaction,
  findProgramAddress,
  SPL_ACCOUNT_LAYOUT,
  TOKEN_PROGRAM_ID,
  InnerSimpleV0Transaction,
  TokenAccount,
} from "@raydium-io/raydium-sdk";

import {
  Connection,
  Keypair,
  PublicKey,
  SendOptions,
  Signer,
  Transaction,
  VersionedTransaction,
} from "@solana/web3.js";
import { addLookupTableInfo, makeTxVersion } from "./config";

export async function buildAndSendTx(
  connection: Connection,
  payer: Signer,
  innerSimpleV0Transaction: InnerSimpleV0Transaction[],
  options?: SendOptions,
) {

  const willSendTx = await buildTxs(connection, payer, innerSimpleV0Transaction);

  return await sendTx(connection, payer, willSendTx, options);
}

export async function buildTxs(
  connection: Connection,
  payer: Signer,
  innerSimpleV0Transaction: InnerSimpleV0Transaction[],
): Promise<(Transaction | VersionedTransaction)[]> {

  let responseBlock;

  try {
      responseBlock = await connection.getLatestBlockhash("confirmed")
  } catch (error) {
      console.log(error)
      console.log("Refetching latest Blockhash")
      responseBlock = await connection.getLatestBlockhash("confirmed")
  }

  const recentBlockhash = (responseBlock).blockhash;

  const transactions = await buildSimpleTransaction({
    connection,
    makeTxVersion,
    payer: payer.publicKey,
    innerTransactions: innerSimpleV0Transaction,
    recentBlockhash,
    addLookupTableInfo: addLookupTableInfo,
  });

  return transactions;
}

export async function sendTx(
  connection: Connection,
  payer: Keypair | Signer,
  txs: (VersionedTransaction | Transaction)[],
  options?: SendOptions,
): Promise<string[]> {
  const txids: string[] = [];
  for (const iTx of txs) {
    if (iTx instanceof VersionedTransaction) {
      iTx.sign([payer]);
      txids.push(await connection.sendTransaction(iTx, options));
    } else {
      txids.push(await connection.sendTransaction(iTx, [payer], options));
    }
  }
  return txids;
}

export async function getWalletTokenAccount(
  connection: Connection,
  wallet: PublicKey
): Promise<TokenAccount[]> {
  const walletTokenAccount = await connection.getTokenAccountsByOwner(wallet, {
    programId: TOKEN_PROGRAM_ID,
  });
  return walletTokenAccount.value.map((i) => ({
    pubkey: i.pubkey,
    programId: i.account.owner,
    accountInfo: SPL_ACCOUNT_LAYOUT.decode(i.account.data),
  }));
}

export function getATAAddress(
  programId: PublicKey,
  owner: PublicKey,
  mint: PublicKey
) {
  const { publicKey, nonce } = findProgramAddress(
    [owner.toBuffer(), programId.toBuffer(), mint.toBuffer()],
    new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
  );
  return { publicKey, nonce };
}

export async function sleepTime(ms: number) {
  console.log(new Date().toLocaleString(), "sleepTime", ms);
  return new Promise((resolve) => setTimeout(resolve, ms));
}
