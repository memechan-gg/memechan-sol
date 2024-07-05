import { ParsedTransactionWithMeta, PublicKey } from "@solana/web3.js";
import { deserializeMetadata, Metadata } from "@metaplex-foundation/mpl-token-metadata";
import { lamports, publicKey } from "@metaplex-foundation/umi";
import { MemechanClient } from "../../MemechanClient";

export type CreateMetadataInstructionParsed = {
  mintAddr: PublicKey;
  metadata: Metadata;
  type: "create_metadata";
};

export async function ParseCreateMetadataInstruction(
  tx: ParsedTransactionWithMeta,
  index: number,
  memechanProgram: MemechanClient,
) {
  const ix = tx.transaction.message.instructions[index];

  if (!("accounts" in ix) || ix.accounts.length < 4) {
    return undefined;
  }

  const mintAddr = ix.accounts[2];
  const metadataAddr = ix.accounts[3];

  const metaAccountInfo = await memechanProgram.connection.getAccountInfo(metadataAddr);

  if (!metaAccountInfo) {
    return undefined;
  }

  const metadata = deserializeMetadata({
    executable: false,
    owner: publicKey(metaAccountInfo.owner),
    lamports: lamports(metaAccountInfo.lamports),
    publicKey: publicKey(metadataAddr),
    data: metaAccountInfo.data,
  });

  if (!metadata) {
    return undefined;
  }

  const cmParsed: CreateMetadataInstructionParsed = {
    mintAddr,
    metadata,
    type: "create_metadata",
  };

  return cmParsed;
}
