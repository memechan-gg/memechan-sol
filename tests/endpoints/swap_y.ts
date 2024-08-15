import { assert, expect } from "chai";
import { BoundPoolWrapper } from "../bound_pool";
import { AccountMeta, Keypair, PublicKey } from "@solana/web3.js";
import {
  createAccount,
  createWrappedNativeAccount,
  getAccount,
} from "@solana/spl-token";
import { mintQuote, payer, provider, sleep } from "../helpers";
import { BN } from "@coral-xyz/anchor";
import { MemeTicketWrapper } from "../ticket";
import {
  DEFAULT_MAX_M,
  DEFAULT_MAX_M_LP,
  DEFAULT_TARGET,
} from "../sol-sdk/config/config";
import { pointsAcc } from "../common";

export function test() {
  describe("swap_y", () => {
    it("swaps full sol->memecoin in one go", async () => {
      const pool = await BoundPoolWrapper.new();
      await mintQuote(payer.publicKey);
      await sleep(1000);
      // call to the swap endpoint
      const ticketId = await pool.swap_y({
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(DEFAULT_TARGET * 10.05),
      });
      sleep(1000);
      const poolInfo = await pool.fetch();
      assert(poolInfo.locked, "pool should be locked");

      const ticketInfo = await ticketId.fetch();
      const memesTotal = ticketInfo.amount.add(poolInfo.adminFeesMeme);
      assert(
        memesTotal.eq(new BN(DEFAULT_MAX_M)),
        `total sum of memetokens with fees expected ${DEFAULT_MAX_M} got ${memesTotal.toString()}`
      );

      const solAmt = poolInfo.quoteReserve.tokens;
      assert(
        solAmt.eq(new BN(DEFAULT_TARGET)),
        `pool expected to have ${DEFAULT_TARGET} quote got ${solAmt.toString()}`
      );

      const solVault = await getAccount(
        provider.connection,
        poolInfo.quoteReserve.vault
      );

      const totalAmt =
        solVault.amount - BigInt(poolInfo.adminFeesQuote.toNumber());

      assert(
        totalAmt === BigInt(DEFAULT_TARGET),
        `pool expected to have ${DEFAULT_TARGET} quote without admin fees got ${totalAmt.toString()}`
      );
    });

    it("swaps with ref", async () => {
      const pool = await BoundPoolWrapper.new();
      const ticketId = await pool.swap_y({
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(DEFAULT_TARGET * 10.05),
        referrer: pointsAcc,
      });
    });

    it("swaps multiple pools for points", async () => {
      for (let i = 0; i < 15; i++) {
        const pool = await BoundPoolWrapper.new();
        const ticketId = await pool.swap_y({
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 10.05),
        });
      }
    });

    it("swaps full sol->memecoin in multiple swaps", async () => {
      const pool = await BoundPoolWrapper.new();

      await sleep(1000);

      const tickets: MemeTicketWrapper[] = [];
      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 0.159),
          ticketNumber: 1,
        })
      );
      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 0.21),
          ticketNumber: 2,
        })
      );

      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 0.21),
          ticketNumber: 3,
        })
      );

      for (let i = 0; i < 4; i++) {
        tickets.push(
          await pool.swap_y({
            memeTokensOut: new BN(1),
            quoteTokensIn: new BN(DEFAULT_TARGET * 0.11),
            ticketNumber: 5 + i,
          })
        );
      }
      console.log("POOL!!", pool.bpClient.id);

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
        sum.add(poolInfo.adminFeesMeme).eq(new BN(DEFAULT_MAX_M)),
        `total sum of memetokens with fees expected ${DEFAULT_MAX_M} got ${sum
          .add(poolInfo.adminFeesMeme)
          .toString()}`
      );

      const solVault = await getAccount(
        provider.connection,
        poolInfo.quoteReserve.vault
      );

      const totalAmt =
        solVault.amount - BigInt(poolInfo.adminFeesQuote.toNumber());
      assert(
        totalAmt.toString() === DEFAULT_TARGET.toString(),
        `pool expected to have ${DEFAULT_TARGET} sol without admin fees got ${totalAmt.toString()}`
      );
    });

    it("user swaps more than have", async () => {
      const pool = await BoundPoolWrapper.new();

      await sleep(1000);

      const user = new Keypair();

      try {
        await pool.swap_y({
          memeTokensOut: new BN(1),
          user: user,
          quoteTokensIn: new BN(50.5 * 1e9),
          userQuoteAcc: await createWrappedNativeAccount(
            provider.connection,
            payer,
            user.publicKey,
            5 * 1e9
          ),
        });
        assert(false, "rpc should have failed");
      } catch (e) {}
    });

    it.skip("swaps many times", async () => {
      const pool = await BoundPoolWrapper.new();
      await mintQuote(payer.publicKey);
      await sleep(1000);

      const ticketsNo = 2000;
      const step: number = 0.1;

      for (let i = 0; i < ticketsNo; i++) {
        const ticketId = await pool.swap_y({
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(step * 10 ** 9),
          ticketNumber: i + 1,
        });
        await sleep(100);
        const ticket = await ticketId.fetch();
        console.log(ticket.amount.toString(), (i + 1) * step);
      }
    });
  });
}
