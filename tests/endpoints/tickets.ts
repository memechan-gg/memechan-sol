import { assert, expect } from "chai";
import { MemeTicket } from "../ticket";
import { BoundPool } from "../bound_pool";
import { BN } from "@project-serum/anchor";
import { airdrop, memechan, payer, provider, sleep } from "../helpers";
import { createWrappedNativeAccount, getAccount } from "@solana/spl-token";
import { Keypair } from "@solana/web3.js"

export function test() {
  describe("merge tickets", () => {
    it("merge tickets presale", async () => {
      const user = Keypair.generate()
      await airdrop(user.publicKey)
      const pool = await BoundPool.new();

      await sleep(1000);

      const userSolAcc = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        500 * 10e9
      );

      const tickets: MemeTicket[] = [];

      tickets.push(await pool.swap_y({
        user,
        userSolAcc,
        memeTokensOut: new BN(1),
        solAmountIn: new BN(50.5 * 1e9),
      }));

      tickets.push(await pool.swap_y({
        user,
        userSolAcc,
        memeTokensOut: new BN(1),
        solAmountIn: new BN(70.7 * 1e9),
      }));

      tickets.push(await pool.swap_y({
        user,
        userSolAcc,
        memeTokensOut: new BN(1),
        solAmountIn: new BN(181.8 * 1e9),
      }));

      await tickets[0].bound_merge({
        pool: pool.id,
        ticketToMerge: tickets[1],
        user: user
      })

      await tickets[0].bound_merge({
        pool: pool.id,
        ticketToMerge: tickets[2],
        user: user
      })
      sleep(1000);
    });

    it("merge tickets live", async () => {
      const user = Keypair.generate()
      await airdrop(user.publicKey)
      const pool = await BoundPool.new();

      await sleep(1000);

      const userSolAcc = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        500 * 10e9
      );

      const tickets: MemeTicket[] = [];

      tickets.push(await pool.swap_y({
        user,
        userSolAcc,
        memeTokensOut: new BN(1),
        solAmountIn: new BN(50.5 * 1e9),
      }));

      tickets.push(await pool.swap_y({
        user,
        userSolAcc,
        memeTokensOut: new BN(1),
        solAmountIn: new BN(70.7 * 1e9),
      }));

      tickets.push(await pool.swap_y({
        user,
        userSolAcc,
        memeTokensOut: new BN(1),
        solAmountIn: new BN(181.8 * 1e9),
      }));

      const [_, staking] = await pool.go_live({})
      sleep(1000);

      await tickets[0].staking_merge({
        staking: staking.id,
        ticketToMerge: tickets[1],
        user: user
      })

      await tickets[0].staking_merge({
        staking: staking.id,
        ticketToMerge: tickets[2],
        user: user
      })
    });

    it("close ticket", async () => {
      const user = Keypair.generate()
      await airdrop(user.publicKey)
      const pool = await BoundPool.new();

      await sleep(1000);

      const userSolAcc = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        500 * 10e9
      );

      const ticket = await pool.swap_y({
        user,
        userSolAcc,
        memeTokensOut: new BN(1),
        solAmountIn: new BN(50.5 * 1e9),
      });

      const ticketInfo = await ticket.fetch();
      await sleep(5000);

      await pool.swap_x({
        user,
        userSolAcc,
        memeAmountIn: ticketInfo.amount,
        userMemeTicket: ticket,
      });

      ticket.close({ user })
    });
  });


}