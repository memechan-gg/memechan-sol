import { assert, expect } from "chai";
import { MemeTicketWrapper } from "../ticket";
import { BoundPoolWrapper } from "../bound_pool";
import { BN } from "@coral-xyz/anchor";
import { airdrop, memechan, payer, provider, sleep } from "../helpers";
import { Keypair } from "@solana/web3.js";
import {
  createWrappedNativeAccount,
  createAssociatedTokenAccount,
  createAccount,
} from "@solana/spl-token";

export function test() {
  describe.skip("staking", () => {
    it.skip("unstake", async () => {
      const users = [
        Keypair.generate(),
        Keypair.generate(),
        Keypair.generate(),
      ];
      const user = users[0];
      await airdrop(user.publicKey);
      const pool = await BoundPoolWrapper.new();

      const tickets: MemeTicketWrapper[] = [];

      tickets.push(
        await pool.swap_y({
          user,
          memeTokensOut: 1,
          quoteTokensIn: 50.5 * 1e9,
        })
      );
      tickets.push(
        await pool.swap_y({
          memeTokensOut: 1,
          quoteTokensIn: 70.7 * 1e9,
        })
      );
      tickets.push(
        await pool.swap_y({
          memeTokensOut: 1,
          quoteTokensIn: 181.8 * 1e9,
        })
      );

      const [amm, staking] = await pool.go_live({});

      const stakingInfo = await staking.fetch();

      const solWalletId = Keypair.generate();
      const solWallet = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        25e9,
        solWalletId
      );

      const memeWalletId = Keypair.generate();
      const memeWallet = await createAccount(
        provider.connection,
        payer,
        stakingInfo.memeMint,
        user.publicKey,
        memeWalletId
      );

      await sleep(1000);

      // await amm.swap(
      //     user,
      //     solWallet,
      //     memeWallet,
      //     20e9,
      //     1
      // );

      staking.unstake({
        ticket: tickets[0],
        user: user,
        amount: (await tickets[0].fetch()).amount,
      });
    });

    // it.skip("withdraw fees", async () => {
    //     const users = [Keypair.generate(), Keypair.generate(), Keypair.generate()];
    //     const user = users[0];
    //     await Promise.all(
    //         users.map((user) => airdrop(user.publicKey)));
    //     const pool = await BoundPool.new();

    //     await sleep(1000);

    //     const tickets: MemeTicket[] = [];

    //     tickets.push(await pool.swap_y({
    //         user: users[0],
    //         memeTokensOut: new BN(1),
    //         solAmountIn: new BN(50.5 * 1e9),
    //     }));
    //     tickets.push(await pool.swap_y({
    //         user: users[1],
    //         memeTokensOut: new BN(1),
    //         solAmountIn: new BN(70.7 * 1e9),
    //     }));
    //     tickets.push(await pool.swap_y({
    //         user: users[2],
    //         memeTokensOut: new BN(1),
    //         solAmountIn: new BN(181.8 * 1e9),
    //     }));

    //     const [amm, staking] = await pool.go_live({});
    //     sleep(1000);

    //     const stakingInfo = await staking.fetch();

    //     const solWalletId = Keypair.generate()
    //     const solWallet = await createWrappedNativeAccount(
    //         provider.connection,
    //         payer,
    //         user.publicKey,
    //         25e9,
    //         solWalletId
    //     )

    //     const memeWalletId = Keypair.generate();
    //     const memeWallet = await createAccount(
    //         provider.connection,
    //         payer,
    //         stakingInfo.memeMint,
    //         user.publicKey,
    //         memeWalletId
    //     );

    //     await sleep(1000);

    //     await amm.swap(
    //         users[0],
    //         solWallet,
    //         memeWallet,
    //         20e9,
    //         1
    //     );

    //     staking.withdraw_fees({
    //         ticket: tickets[0],
    //         user: users[0],
    //     })
    // });
  });
}
