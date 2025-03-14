import BN from "bn.js";
import { QUOTE_MINT, adminSigner, airdrop, mintQuote, payer } from "./helpers";
import { TargetConfig } from "./sol-sdk/targetconfig/TargetConfig";
import { client } from "./common";
import { MEMECHAN_QUOTE_TOKEN, memechan } from "./sol-sdk/config/config";

export class TargetConfigWrapper {
  static async new(target_amt: number = 90_000_000_000) {
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
