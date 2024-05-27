import { QUOTE_MINT, memechan, payer, provider, sleep } from "../helpers";
import { BN } from "@coral-xyz/anchor";
import { client } from "../common";
import { Token } from "@raydium-io/raydium-sdk";
import { BoundPoolWrapper } from "../bound_pool";

export function test() {
  describe("go_live", () => {
    it("full swap then go live", async () => {
      const pool = await BoundPoolWrapper.new();

      await sleep(500);

      const ticketId = await pool.swap_y({
        memeTokensOut: 1,
        quoteTokensIn: 303 * 1e9,
      });

      await pool.go_live({});
    });
  });
}
