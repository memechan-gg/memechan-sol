import { assert, expect } from "chai";
import { BoundPoolWrapper } from "../bound_pool";
import { AccountMeta, Keypair, PublicKey } from "@solana/web3.js";
import {
  createAccount,
  createWrappedNativeAccount,
  getAccount,
} from "@solana/spl-token";
import { memechan, payer, provider, sleep } from "../helpers";
import { BN } from "@coral-xyz/anchor";

export function test() {
  describe.skip("swap_x", () => {
    it.skip("swaps user sol->memecoin->sol", async () => {
      const user = Keypair.generate();
      const pool = await BoundPoolWrapper.new();

      const userSolAcc = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        500 * 10e9
      );

      await sleep(1000);

      const ticketId = await pool.swap_y({
        user,
        memeTokensOut: 1,
        quoteTokensIn: 30 * 1e9,
      });

      await sleep(6000);

      // await pool.swap_x({
      //   user,
      //   userMemeTicket: ticketId,
      //   userSolAcc
      // })
    });

    it.skip("swaps sol->memecoin->sol->full meme", async () => {
      const user = Keypair.generate();
      const pool = await BoundPoolWrapper.new();

      const userSolAcc = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        500 * 10e9
      );

      await sleep(1000);

      const userMemeTicket = await pool.swap_y({
        user,
        memeTokensOut: 1,
        quoteTokensIn: 30 * 1e9,
      });

      await sleep(6000);

      // await pool.swap_x({
      //   user,
      //   userMemeTicket,
      //   userSolAcc
      // })

      const ticketId = await pool.swap_y({
        memeTokensOut: 1,
        quoteTokensIn: 303 * 1e9,
      });

      await sleep(1000);

      const poolInfo = await pool.fetch();

      assert(poolInfo.locked, "pool should be locked");

      const ticketOneInfo = await userMemeTicket.fetch();
      const ticketInfo = await ticketId.fetch();

      const memesTotal = ticketInfo.amount
        .add(ticketOneInfo.amount)
        .add(poolInfo.adminFeesMeme);
      assert(
        memesTotal.eq(new BN(9e14)),
        "total sum of memetokens with fees should amount to 9e14"
      );

      const solAmt = poolInfo.quoteReserve.tokens;
      assert(solAmt.eq(new BN(3e11)), "pool should have 300 sol");

      const solVault = await getAccount(
        provider.connection,
        poolInfo.quoteReserve.vault
      );

      const totalAmt =
        solVault.amount - BigInt(poolInfo.adminFeesQuote.toNumber());
      assert(
        totalAmt === BigInt(3e11),
        "pool should have 300 sol without admin fees"
      );
    });
  });
}
