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
import { MemeTicketWrapper } from "../ticket";

export function test() {
  describe.skip("swap_y", () => {
    it.skip("swaps full sol->memecoin in one go", async () => {
      const pool = await BoundPoolWrapper.new();

      await sleep(1000);

      // call to the swap endpoint
      const ticketId = await pool.swap_y({
        memeTokensOut: new BN(1),
        solAmountIn: new BN(303 * 1e9),
      });

      sleep(1000);

      const poolInfo = await pool.fetch();

      assert(poolInfo.locked, "pool should be locked");

      const ticketInfo = await ticketId.fetch();

      const memesTotal = ticketInfo.amount.add(poolInfo.adminFeesMeme);
      assert(
        memesTotal.eq(new BN(9e14)),
        "total sum of memetokens with fees should amount to 9e14"
      );

      const solAmt = poolInfo.memeReserve.tokens;
      assert(solAmt.eq(new BN(3e11)), "pool should have 300 sol");

      const solVault = await getAccount(
        provider.connection,
        poolInfo.memeReserve.vault
      );

      const totalAmt =
        solVault.amount - BigInt(poolInfo.adminFeesQuote.toNumber());
      assert(
        totalAmt === BigInt(3e11),
        "pool should have 300 sol without admin fees"
      );
    });

    it.skip("swaps full sol->memecoin in multiple swaps", async () => {
      const pool = await BoundPoolWrapper.new();

      await sleep(1000);

      const tickets: MemeTicketWrapper[] = [];

      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          solAmountIn: new BN(50.5 * 1e9),
        })
      );

      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          solAmountIn: new BN(70.7 * 1e9),
        })
      );

      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          solAmountIn: new BN(181.8 * 1e9),
        })
      );

      sleep(1000);

      const poolInfo = await pool.fetch();

      assert(poolInfo.locked, "pool should be locked");

      let sum = new BN(0);
      for (let i = 0; i < tickets.length; i++) {
        const ticket1Id = tickets[i];

        const ticketInfo = await ticket1Id.fetch();
        sum = sum.add(ticketInfo.amount);
      }

      assert(
        sum.add(poolInfo.adminFeesMeme).eq(new BN(9e14)),
        "total sum of memetokens with fees should amount to 9e14"
      );

      const solVault = await getAccount(
        provider.connection,
        poolInfo.memeReserve.vault
      );

      const totalAmt =
        solVault.amount - BigInt(poolInfo.adminFeesQuote.toNumber());
      assert(
        totalAmt === BigInt(3e11),
        "pool should have 300 sol without admin fees"
      );
    });

    it.skip("user swaps more than have", async () => {
      const pool = await BoundPoolWrapper.new();

      await sleep(1000);

      const user = new Keypair();

      try {
        await pool.swap_y({
          memeTokensOut: new BN(1),
          user: user,
          solAmountIn: new BN(50.5 * 1e9),
          userSolAcc: await createWrappedNativeAccount(
            provider.connection,
            payer,
            user.publicKey,
            5 * 1e9
          ),
        });
        assert(false, "rpc should have failed");
      } catch (e) {}
    });
  });
}
