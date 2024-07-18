import {
  QUOTE_MINT,
  airdrop,
  airdrop_tokens,
  mintQuote,
  payer,
  provider,
  sleep,
} from "../helpers";
import { BN } from "@coral-xyz/anchor";
import { client } from "../common";
import { BoundPoolWrapper } from "../bound_pool";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

export function test() {
  describe("go_live", () => {
    it("full swap then go live", async () => {
      const pool = await BoundPoolWrapper.new();

      await sleep(500);

      const addr = await getOrCreateAssociatedTokenAccount(
        client.connection,
        payer,
        QUOTE_MINT,
        payer.publicKey
      );
      await mintQuote(addr.address);

      const ticketId = await pool.swap_y({
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(50000 * 1e9),
      });

      await pool.go_live();
    });

    it("full swap then go live with airdrop", async () => {
      const pool = await BoundPoolWrapper.new(10, true);

      await sleep(500);

      const addr = await getOrCreateAssociatedTokenAccount(
        client.connection,
        payer,
        QUOTE_MINT,
        payer.publicKey
      );
      await mintQuote(addr.address);

      const ticketId = await pool.swap_y({
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(50000 * 1e9),
      });

      await pool.go_live();
    });
  });
}
