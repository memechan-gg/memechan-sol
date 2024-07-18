import { MemeTicketWrapper } from "../ticket";
import { BoundPoolWrapper } from "../bound_pool";
import { BN } from "@coral-xyz/anchor";
import { airdrop, payer, provider, sleep } from "../helpers";
import { createWrappedNativeAccount, getAccount } from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { DEFAULT_TARGET } from "../sol-sdk/config/config";

export function test() {
  describe("merge tickets", () => {
    it("merge tickets presale", async () => {
      const user = Keypair.generate();
      await airdrop(user.publicKey);
      const pool = await BoundPoolWrapper.new();
      await sleep(1000);

      const userSolAcc = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        500 * 10e9
      );

      const tickets: MemeTicketWrapper[] = [];

      tickets.push(
        await pool.swap_y({
          user,
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 0.25),
          ticketNumber: 1,
        })
      );

      tickets.push(
        await pool.swap_y({
          user,
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 0.35),
          ticketNumber: 2,
        })
      );

      tickets.push(
        await pool.swap_y({
          user,
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 0.4),
          ticketNumber: 3,
        })
      );

      await tickets[0].bound_merge({
        pool: pool.bpClient.id,
        ticketToMerge: tickets[1],
        user: user,
      });

      await tickets[0].bound_merge({
        pool: pool.bpClient.id,
        ticketToMerge: tickets[2],
        user: user,
      });
      sleep(1000);
    });

    it("merge tickets live", async () => {
      const user = Keypair.generate();
      await airdrop(user.publicKey);
      const pool = await BoundPoolWrapper.new();

      await sleep(1000);
      const userSolAcc = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        500 * 10e9
      );

      const tickets: MemeTicketWrapper[] = [];

      tickets.push(
        await pool.swap_y({
          user,
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 0.25),
          ticketNumber: 1,
        })
      );

      tickets.push(
        await pool.swap_y({
          user,
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 0.45),
          ticketNumber: 2,
        })
      );

      tickets.push(
        await pool.swap_y({
          user,
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(DEFAULT_TARGET * 0.65),
          ticketNumber: 3,
        })
      );

      const [amm1, amm2, staking] = await pool.go_live();
      sleep(1000);

      await tickets[0].staking_merge({
        staking: staking.id,
        ticketToMerge: tickets[1],
        user: user,
      });

      await tickets[0].staking_merge({
        staking: staking.id,
        ticketToMerge: tickets[2],
        user: user,
      });
    });

    it("close ticket", async () => {
      const user = Keypair.generate();
      await airdrop(user.publicKey);
      const pool = await BoundPoolWrapper.new();

      await sleep(1000);
      const userSolAcc = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        500 * 10e9
      );

      const ticket = await pool.swap_y({
        user,
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(DEFAULT_TARGET * 0.5),
      });

      const ticketInfo = await ticket.fetch();
      await sleep(5000);

      await pool.swap_x({
        user,
        memeAmountIn: ticketInfo.amount,
        userMemeTicket: ticket,
        userQuoteAcc: userSolAcc,
      });

      ticket.close({ user });
    });
  });
}
