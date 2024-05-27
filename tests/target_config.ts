import BN from "bn.js";
import {
  QUOTE_MINT,
  adminSigner,
  airdrop,
  memechan,
  mintQuote,
  payer,
} from "./helpers";
import { TargetConfig } from "./sol-sdk/targetconfig/TargetConfig";
import { client } from "./common";
import { MEMECHAN_QUOTE_TOKEN } from "./sol-sdk/config/config";

export class TargetConfigWrapper {
  static async new(target_amt: number = 40_000_000_000_000) {
    const tcdata = await memechan.account.targetConfig.fetchNullable(
      TargetConfig.findTargetConfigPda(QUOTE_MINT, memechan.programId)
    );

    if (tcdata !== null) {
      return;
    }

    await airdrop(adminSigner.publicKey);

    await TargetConfig.new({
      client,
      mint: QUOTE_MINT,
      payer: adminSigner,
      targetAmount: new BN(target_amt),
    });
  }
}
