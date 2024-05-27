import {
  MarketV2,
  TOKEN_PROGRAM_ID,
  Token,
  generatePubKey,
} from "@raydium-io/raydium-sdk";
import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import { PROGRAMIDS, makeTxVersion } from "./config";
import { sendTx } from "./utils";
import { buildTxs } from "../util";

type CreateMarketTxInput = {
  baseToken: Token;
  quoteToken: Token;
  wallet: PublicKey;
  signer: Keypair;
  connection: Connection;
};

export async function createMarket(input: CreateMarketTxInput) {
  const { transactions: createMarketTransactions, marketId } =
    await getCreateMarketTransactions(input);

  console.log(
    "createMarketTransactions:",
    JSON.stringify(createMarketTransactions)
  );

  return {
    txids: await sendTx(
      input.connection,
      input.signer,
      createMarketTransactions,
      {
        skipPreflight: true,
      }
    ),
    marketId,
  };
}

export async function getCreateMarketTransactions(
  input: CreateMarketTxInput
): Promise<{
  transactions: (Transaction | VersionedTransaction)[];
  marketId: PublicKey;
}> {
  console.log("PROGRAMIDS.OPENBOOK_MARKET ", PROGRAMIDS.OPENBOOK_MARKET);

  const market = generatePubKey({
    fromPublicKey: input.wallet,
    programId: PROGRAMIDS.OPENBOOK_MARKET,
  });
  const requestQueue = generatePubKey({
    fromPublicKey: input.wallet,
    programId: PROGRAMIDS.OPENBOOK_MARKET,
  });
  const eventQueue = generatePubKey({
    fromPublicKey: input.wallet,
    programId: PROGRAMIDS.OPENBOOK_MARKET,
  });
  const bids = generatePubKey({
    fromPublicKey: input.wallet,
    programId: PROGRAMIDS.OPENBOOK_MARKET,
  });
  const asks = generatePubKey({
    fromPublicKey: input.wallet,
    programId: PROGRAMIDS.OPENBOOK_MARKET,
  });
  const baseVault = generatePubKey({
    fromPublicKey: input.wallet,
    programId: TOKEN_PROGRAM_ID,
  });
  const quoteVault = generatePubKey({
    fromPublicKey: input.wallet,
    programId: TOKEN_PROGRAM_ID,
  });

  console.log("market: ", market.publicKey);
  console.log("requestQueue: ", requestQueue.publicKey);
  console.log("eventQueue: ", eventQueue.publicKey);
  console.log("bids: ", bids.publicKey);
  console.log("asks: ", asks.publicKey);
  console.log("baseVault: ", baseVault.publicKey);
  console.log("quoteVault: ", quoteVault.publicKey);

  const createMarketInstruments =
    await MarketV2.makeCreateMarketInstructionSimple({
      connection: input.connection,
      wallet: input.wallet,
      baseInfo: input.baseToken,
      quoteInfo: input.quoteToken,
      // set based on https://docs.raydium.io/raydium/updates/archive/creating-an-openbook-amm-pool
      lotSize: 1,
      tickSize: 0.000001,
      dexProgramId: PROGRAMIDS.OPENBOOK_MARKET,
      makeTxVersion,
    });

  const transactions = await buildTxs(
    input.connection,
    input.signer.publicKey,
    createMarketInstruments.innerTransactions
  );

  return { transactions, marketId: createMarketInstruments.address.marketId };
}

export function getCreateMarketInstructions(
  transactions: (Transaction | VersionedTransaction)[]
): TransactionInstruction[] {
  const instructions: TransactionInstruction[] = [];

  transactions.forEach((tx) => {
    if (tx instanceof VersionedTransaction) {
      const txMessage = TransactionMessage.decompile(tx.message);
      const txInstructions = txMessage.instructions;
      instructions.push(...txInstructions);
    } else {
      instructions.push(...tx.instructions);
    }
  });

  return instructions;
}
