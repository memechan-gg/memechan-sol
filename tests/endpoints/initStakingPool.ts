import { assert, expect } from "chai";
import { AccountMeta, Keypair, PublicKey } from "@solana/web3.js";
import {
  createAccount,
  createWrappedNativeAccount,
  getAccount,
} from "@solana/spl-token";
import { memechan, payer, provider, sleep } from "../helpers";
import { BN } from "@project-serum/anchor";
import { MemeTicket } from "../ticket";
import { BoundPool } from "../boundPool";
import { DEFAULT_MAX_M } from "../config";

export function test() {
  describe("init_staking_pool", () => {
    let pool: BoundPool;
    let pooolInfo;
    let user = Keypair.generate();
    let ticket: MemeTicket;

    beforeEach("init boundpool", async () => {
      pool = await BoundPool.new({});
      pooolInfo = await pool.fetch();
    });

    beforeEach("swaps full quote -> meme", async () => {
      // call to the swap endpoint
      ticket = await pool.swapY({
        owner: user,
        coinXMinValue: new BN(1),
        coinInAmount: new BN(45_000 * 1e9), // target is 40_000
      });
    });

    it("init staking pool", async () => {
      const poolInfo = await pool.fetch();
      assert(poolInfo.locked, "pool should be locked");
      const ticketInfo = await ticket.fetch();

      await pool.initStakingPool({
        signer: user,
      });
    });
  });
}
