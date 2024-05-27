import { TransactionSignature } from "@solana/web3.js";
import * as base58 from "bs58";

import * as solana from "@solana/web3.js";
import { MemechanClient } from "../MemechanClient";
import { NewBPInstructionParsed, ParseNewBPInstruction } from "./parsers/bonding-pool-creation-parser";
import { GoLiveInstructionParsed, ParseGoLiveInstruction } from "./parsers/go-live-parser";
import { InitStakingPoolInstructionParsed, ParseInitStakingInstruction } from "./parsers/init-staking-parser";
import { ParseSwapXInstruction, SwapXInstructionParsed } from "./parsers/swap-x-parser";
import { ParseSwapYInstruction, SwapYInstructionParsed } from "./parsers/swap-y-parser";
import { CreateMetadataInstructionParsed, ParseCreateMetadataInstruction } from "./parsers/create-metadata-parser";

export async function ParseTx(
  txSig: TransactionSignature,
  client: MemechanClient,
): Promise<
  | (
      | NewBPInstructionParsed
      | SwapYInstructionParsed
      | SwapXInstructionParsed
      | InitStakingPoolInstructionParsed
      | GoLiveInstructionParsed
      | CreateMetadataInstructionParsed
    )[]
  | undefined
> {
  const pt = await client.connection.getParsedTransaction(txSig);
  //console.log(pt);

  const ixs = pt?.transaction?.message.instructions;
  if (!ixs) {
    return undefined;
  }

  const res: (
    | NewBPInstructionParsed
    | SwapYInstructionParsed
    | SwapXInstructionParsed
    | InitStakingPoolInstructionParsed
    | GoLiveInstructionParsed
    | CreateMetadataInstructionParsed
  )[] = [];

  for (let i = 0; i < ixs.length; i++) {
    const ix = ixs[i];
    if (ix.programId.equals(client.memechanProgram.programId)) {
      if ("data" in ix) {
        const ixBytes = base58.decode(ix.data);
        const buffer = Buffer.from(ixBytes);
        const pres = await ptx(buffer, pt, i, client);
        if (pres) {
          res.push(pres);
        }
      }
    } else continue;
  }

  return res;
}

async function ptx(
  ixBytes: Buffer,
  tx: solana.ParsedTransactionWithMeta,
  index: number,
  memechanProgram: MemechanClient,
): Promise<
  | NewBPInstructionParsed
  | SwapYInstructionParsed
  | SwapXInstructionParsed
  | InitStakingPoolInstructionParsed
  | GoLiveInstructionParsed
  | CreateMetadataInstructionParsed
  | undefined
> {
  const ixBytesSliced = ixBytes.subarray(0, 2);

  if (ixBytesSliced.equals(Buffer.from([0x26, 0x3f]))) {
    console.log("parsing ix: NewPool");
    return await ParseNewBPInstruction(tx, index, memechanProgram);
  }
  if (ixBytesSliced.equals(Buffer.from([0x41, 0x3f]))) {
    console.log("parsing ix: SwapX");
    return await ParseSwapXInstruction(tx, index, memechanProgram);
  }
  if (ixBytesSliced.equals(Buffer.from([0x7e, 0xd0]))) {
    console.log("parsing ix: SwapY");
    return await ParseSwapYInstruction(tx, index, memechanProgram);
  }
  if (ixBytesSliced.equals(Buffer.from([0x68, 0xc1]))) {
    console.log("parsing ix: InitStaking");
    return await ParseInitStakingInstruction(tx, index, memechanProgram);
  }
  if (ixBytesSliced.equals(Buffer.from([0x7e, 0x62]))) {
    console.log("parsing ix: GoLive");
    return await ParseGoLiveInstruction(tx, index);
  }
  if (ixBytesSliced.equals(Buffer.from([0x1e, 0x23]))) {
    console.log("parsing ix: CreateMetadata");
    return await ParseCreateMetadataInstruction(tx, index, memechanProgram);
  }

  return undefined;
}
