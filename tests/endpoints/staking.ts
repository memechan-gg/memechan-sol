import { MemeTicketWrapper } from "../ticket";
import { BoundPoolWrapper } from "../bound_pool";
import { BN } from "@coral-xyz/anchor";
import {
  QUOTE_MINT,
  airdrop,
  mintChan,
  mintQuote,
  payer,
  provider,
  sleep,
} from "../helpers";
import { Keypair, PublicKey, Transaction } from "@solana/web3.js";
import {
  createWrappedNativeAccount,
  createAssociatedTokenAccount,
  createAccount,
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  NATIVE_MINT,
} from "@solana/spl-token";
import BigNumber from "bignumber.js";
import { CHAN_TOKEN_INFO, DEFAULT_TARGET } from "../sol-sdk/config/config";
import { wrapSOLInstruction } from "@mercurial-finance/dynamic-amm-sdk/dist/cjs/src/amm/utils";
import { MemeTicket } from "../sol-sdk/memeticket/MemeTicket";
import { BE_AUTH, LP_FEE_VAULT_OWNER } from "../common";
import assert from "assert";

export function test() {
  describe("staking", () => {
    it("unstake", async () => {
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
          quoteTokensIn: new BN(0.152 * DEFAULT_TARGET),
        })
      );
      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(0.253 * DEFAULT_TARGET),
          ticketNumber: 1,
        })
      );
      tickets.push(
        await pool.swap_y({
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(0.75 * DEFAULT_TARGET),
          ticketNumber: 2,
        })
      );

      const [amm, amm2, staking] = await pool.go_live();

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

      await amm.swap(user, 10e9, 1);

      await sleep(18000);

      await staking.unstake({
        ticket: tickets[0],
        user: user,
        amount: (await tickets[0].fetch()).amount,
      });
    });

    it("withdraw fees", async () => {
      const users = [
        Keypair.generate(),
        Keypair.generate(),
        Keypair.generate(),
      ];
      const user = users[0];
      await Promise.all(users.map((user) => airdrop(user.publicKey)));
      const pool = await BoundPoolWrapper.new();

      const addr = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        QUOTE_MINT,
        payer.publicKey
      );
      if (QUOTE_MINT.equals(NATIVE_MINT)) {
        const tx = new Transaction();
        tx.add(
          ...wrapSOLInstruction(
            payer.publicKey,
            addr.address,
            BigInt(100_000_000_000)
          )
        );
        await provider.connection.sendTransaction(tx, [payer], {
          skipPreflight: true,
        });
      } else {
        await mintQuote(addr.address);
      }

      await sleep(1000);

      const tickets: MemeTicketWrapper[] = [];

      tickets.push(
        await pool.swap_y({
          user: users[0],
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(0.152 * DEFAULT_TARGET),
        })
      );
      tickets.push(
        await pool.swap_y({
          user: users[1],
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(0.253 * DEFAULT_TARGET),
        })
      );
      tickets.push(
        await pool.swap_y({
          user: users[2],
          memeTokensOut: new BN(1),
          quoteTokensIn: new BN(0.75 * DEFAULT_TARGET),
        })
      );
      sleep(500);
      const [amm, amm2, staking] = await pool.go_live();
      sleep(500);

      const stakingInfo = await staking.fetch();
      const memeWalletId = Keypair.generate();
      const memeWallet = await createAccount(
        provider.connection,
        payer,
        stakingInfo.memeMint,
        user.publicKey,
        memeWalletId
      );
      await sleep(200);
      const inputTokenAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        stakingInfo.memeMint,
        payer.publicKey
      );

      const chanTokenAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        new PublicKey(CHAN_TOKEN_INFO.address),
        payer.publicKey
      );
      console.log(6);
      await mintChan(chanTokenAccount.address);
      await amm.swap(payer, 20e9, 1);
      await amm2.swap(payer, 5e9, 1);
      await staking.add_fees(amm, amm2);

      console.log("withdrawing user fees");
      await staking.withdraw_fees({
        ticket: tickets[0],
        user: users[0].publicKey,
      });

      await sleep(500);
      const fetchedTicket = await tickets[0].fetch();
      console.log(
        "ticket % ",
        BigNumber(fetchedTicket.amount.toString())
          .div(BigNumber(stakingInfo.stakesTotal.toString()))
          .toString()
      );

      const adminTicket = await new MemeTicketWrapper(
        staking.findAdminMemeTicket()
      );

      const beMeme = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        stakingInfo.memeMint,
        BE_AUTH
      );

      console.log("trying to withdrawing admin fees with wrong be");
      try {
        await staking.withdraw_fees({
          ticket: adminTicket,
          user: LP_FEE_VAULT_OWNER,
          beMeme: inputTokenAccount.address,
        });
        assert(false, "rpc should have failed");
      } catch (e) {}

      console.log("trying to withdrawing admin fees without be");
      try {
        inputTokenAccount;
        await staking.withdraw_fees({
          ticket: adminTicket,
          user: LP_FEE_VAULT_OWNER,
        });
        assert(false, "rpc should have failed");
      } catch (e) {}

      console.log("withdrawing admin fees");
      await staking.withdraw_fees({
        ticket: adminTicket,
        user: LP_FEE_VAULT_OWNER,
        beMeme: beMeme.address,
      });

      console.log(toString(await staking.fetch()));

      console.log(toString(await adminTicket.fetch()));
    });
  });
}

function toString(obj): string {
  let ln = "o";
  for (const property in obj) {
    ln += `${property}: ${obj[property]}\n`;
  }

  return ln;
}
