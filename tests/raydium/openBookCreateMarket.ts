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
import { makeTxVersion } from "./config";
import { sendTx } from "./utils";
import { buildTxs } from "../util";
import { openbookPubkey, raydiumPubkey } from "../config";

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
  const openbookProgram = openbookPubkey();

  const market = generatePubKey({
    fromPublicKey: input.wallet,
    programId: openbookProgram,
  });
  const requestQueue = generatePubKey({
    fromPublicKey: input.wallet,
    programId: openbookProgram,
  });
  const eventQueue = generatePubKey({
    fromPublicKey: input.wallet,
    programId: openbookProgram,
  });
  const bids = generatePubKey({
    fromPublicKey: input.wallet,
    programId: openbookProgram,
  });
  const asks = generatePubKey({
    fromPublicKey: input.wallet,
    programId: openbookProgram,
  });
  const baseVault = generatePubKey({
    fromPublicKey: input.wallet,
    programId: TOKEN_PROGRAM_ID,
  });
  const quoteVault = generatePubKey({
    fromPublicKey: input.wallet,
    programId: TOKEN_PROGRAM_ID,
  });

  const createMarketInstruments =
    await MarketV2.makeCreateMarketInstructionSimple({
      connection: input.connection,
      wallet: input.wallet,
      baseInfo: input.baseToken,
      quoteInfo: input.quoteToken,
      // set based on https://docs.raydium.io/raydium/updates/archive/creating-an-openbook-amm-pool
      lotSize: 1,
      tickSize: 0.000001,
      dexProgramId: openbookProgram,
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
