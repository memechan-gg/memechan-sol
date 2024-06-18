import { assert, expect } from "chai";
import { MemeTicketWrapper } from "../ticket";
import { BoundPoolWrapper } from "../bound_pool";
import { BN } from "@coral-xyz/anchor";
import {
  QUOTE_MINT,
  airdrop,
  memechan,
  mintQuote,
  payer,
  provider,
  sleep,
} from "../helpers";
import { Keypair, Transaction } from "@solana/web3.js";
import {
  createWrappedNativeAccount,
  createAssociatedTokenAccount,
  createAccount,
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  NATIVE_MINT,
} from "@solana/spl-token";
import { associatedAddress } from "@coral-xyz/anchor/dist/cjs/utils/token";
import BigNumber from "bignumber.js";
import { wrapSOLInstruction } from "@mercurial-finance/vault-sdk/dist/cjs/src/vault/utils";

export function test() {
  describe("staking", () => {
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
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(6000 * 1e9),
        })
      );
      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(8070 * 1e9),
        })
      );
      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(28180 * 1e9),
        })
      );

      const [amm, staking] = await pool.go_live();

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

      await amm.swap(user, 20e9, 1);

      staking.unstake({
        ticket: tickets[0],
        user: user,
        amount: (await tickets[0].fetch()).amount,
      });
    });

    it("withdraw fees", async () => {
      console.log("0");
      const users = [
        Keypair.generate(),
        Keypair.generate(),
        Keypair.generate(),
      ];
      const user = users[0];
      await Promise.all(users.map((user) => airdrop(user.publicKey)));
      const pool = await BoundPoolWrapper.new();
      console.log("\nbpclient\n", pool.bpClient.id);

      const addr = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        QUOTE_MINT,
        payer.publicKey
      );
      console.log("-1");
      if (QUOTE_MINT.equals(NATIVE_MINT)) {
        console.log("0");
        const tx = new Transaction();
        tx.add(
          ...wrapSOLInstruction(
            payer.publicKey,
            addr.address,
            new BN(100_000_000_000)
          )
        );
        await provider.connection.sendTransaction(tx, [payer], {
          skipPreflight: true,
        });
      } else {
        await mintQuote(addr.address);
      }

      console.log("1");
      await sleep(1000);

      const tickets: MemeTicketWrapper[] = [];

      tickets.push(
        await pool.swap_y({
          user: users[0],
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(20 * 1e9),
        })
      );
      tickets.push(
        await pool.swap_y({
          user: users[1],
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(30 * 1e9),
        })
      );
      tickets.push(
        await pool.swap_y({
          user: users[2],
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(50 * 1e9),
        })
      );
      sleep(500);
      console.log("-2");
      const [amm, staking] = await pool.go_live();
      sleep(1000);

      const stakingInfo = await staking.fetch();

      console.log("-1");
      const memeWalletId = Keypair.generate();
      const memeWallet = await createAccount(
        provider.connection,
        payer,
        stakingInfo.memeMint,
        user.publicKey,
        memeWalletId
      );

      await sleep(1000);
      console.log("0");

      const inputTokenAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        stakingInfo.memeMint,
        payer.publicKey
      );

      console.log("1");
      await amm.swap(payer, 20e9, 1);
      console.log("2");

      await staking.add_fees(amm);

      await staking.withdraw_fees({
        ticket: tickets[0],
        user: users[0],
      });
      const fetchedTicket = await tickets[0].fetch();
      console.log(
        "ticket % ",
        BigNumber(fetchedTicket.amount.toString())
          .div(BigNumber(stakingInfo.stakesTotal.toString()))
          .toString()
      );
    });
  });
}
