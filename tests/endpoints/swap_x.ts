import { assert, expect } from "chai";
import { BoundPoolWrapper } from "../bound_pool";
import { AccountMeta, Keypair, PublicKey } from "@solana/web3.js";
import {
  createAccount,
  createWrappedNativeAccount,
  getAccount,
} from "@solana/spl-token";
import { QUOTE_MINT, airdrop, payer, provider, sleep } from "../helpers";
import { BN } from "@coral-xyz/anchor";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { client } from "../common";
import { DEFAULT_MAX_M, DEFAULT_TARGET } from "../sol-sdk/config/config";
import { TargetConfig } from "../sol-sdk/targetconfig/TargetConfig";

export function test() {
  describe("swap_x", () => {
    it("swaps user sol->memecoin->sol", async () => {
      const user = Keypair.generate();
      const pool = await BoundPoolWrapper.new();

      await airdrop(user.publicKey);

      const userQuoteAcc = (
        await getOrCreateAssociatedTokenAccount(
          client.connection,
          payer,
          QUOTE_MINT,
          user.publicKey
        )
      ).address;

      await sleep(1000);
      const ticketId = await pool.swap_y({
        user,
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(30 * 1e9),
      });

      await sleep(6000);
      await pool.swap_x({
        user,
        userMemeTicket: ticketId,
        userQuoteAcc,
      });
    });

    it("swaps sol->memecoin->sol->full meme", async () => {
      const user = Keypair.generate();
      const pool = await BoundPoolWrapper.new();

      const userQuoteAcc = (
        await getOrCreateAssociatedTokenAccount(
          client.connection,
          payer,
          QUOTE_MINT,
          user.publicKey
        )
      ).address;

      await airdrop(user.publicKey);

      await sleep(1000);

      const userMemeTicket = await pool.swap_y({
        user,
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(30 * 1e9),
      });

      await sleep(6000);

      await pool.swap_x({
        user,
        userMemeTicket,
        userQuoteAcc,
      });

      const ticketId = await pool.swap_y({
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(304 * 1e9),
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
        memesTotal.eq(new BN(DEFAULT_MAX_M)),
        `total sum of memetokens with fees should amount to ${DEFAULT_MAX_M} got ${memesTotal.toString()}`
      );

      const solAmt = poolInfo.quoteReserve.tokens;
      assert(
        solAmt.eq(new BN(DEFAULT_TARGET)),
        `pool should have ${DEFAULT_TARGET} sol got ${solAmt.toString()}`
      );

      const solVault = await getAccount(
        provider.connection,
        poolInfo.quoteReserve.vault
      );

      const totalAmt =
        solVault.amount - BigInt(poolInfo.adminFeesQuote.toNumber());
      assert(
        totalAmt === BigInt(DEFAULT_TARGET),
        `pool should have ${DEFAULT_TARGET}} sol without admin fees got ${totalAmt.toString()}`
      );
    });
  });
}
