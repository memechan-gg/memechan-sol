import BN from "bn.js";
import { airdrop, mintChan, payer, provider, sleep } from "../helpers";
import { AmmPool } from "../pool";
import {
  createAssociatedTokenAccount,
  createWrappedNativeAccount,
  getOrCreateAssociatedTokenAccount,
  NATIVE_MINT,
  syncNative,
  transfer,
} from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { StakingWrapper } from "../staking";
import { BoundPoolWrapper } from "../bound_pool";
import { CHAN_TOKEN_INFO, DEFAULT_TARGET } from "../sol-sdk/config/config";

export function test() {
  describe("fees", () => {
    it("withdraw fees", async () => {
      const user = Keypair.generate();
      await airdrop(user.publicKey);

      const boundPool = await BoundPoolWrapper.new();

      await sleep(1000);

      // call to the swap endpoint
      const ticketId = await boundPool.swap_y({
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(DEFAULT_TARGET * 1.05),
      });

      const poolInfo = await boundPool.fetch();

      sleep(1000);

      const [amm, amm2, staking] = await boundPool.go_live();

      const solWallet = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        NATIVE_MINT,
        user.publicKey
      );

      await sendAndConfirmTransaction(
        provider.connection,
        new Transaction().add(
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: solWallet.address,
            lamports: 10e9,
          })
        ),
        [payer]
      );

      await syncNative(provider.connection, user, solWallet.address);

      const memeWallet = await createAssociatedTokenAccount(
        provider.connection,
        payer,
        poolInfo.memeReserve.mint,
        user.publicKey
      );

      const chanTokenAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        new PublicKey(CHAN_TOKEN_INFO.address),
        user.publicKey
      );

      await mintChan(chanTokenAccount.address);
      await amm.swap(user, 1e9, 1);
      await amm2.swap(user, 1e9, 1);

      await staking.add_fees(amm, amm2);
    });
  });
}
