import { QUOTE_MINT, memechan, payer, provider, sleep } from "../helpers";
import { BN } from "@coral-xyz/anchor";
import { client } from "../common";
import { Token } from "@raydium-io/raydium-sdk";
import { BoundPoolWrapper } from "../bound_pool";

export function test() {
  describe.skip("go_live", () => {
    it.skip("full swap then go live", async () => {
      const pool = await BoundPoolWrapper.new();

      await sleep(500);

      const ticketId = await pool.swap_y({
        memeTokensOut: new BN(1),
        solAmountIn: new BN(303 * 1e9),
      });

      await pool.go_live({});
    });
  });
}
