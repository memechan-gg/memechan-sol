import BN from "bn.js";
import { airdrop, payer, provider, sleep } from "../helpers";
import { AmmPool } from "../pool";
import {
  createAssociatedTokenAccount,
  createWrappedNativeAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { StakingWrapper } from "../staking";
import { BoundPoolWrapper } from "../bound_pool";

export function test() {
  describe.skip("fees", () => {
    it.skip("withdraw fees", async () => {
      const user = Keypair.generate();
      await airdrop(user.publicKey);

      const boundPool = await BoundPoolWrapper.new();

      await sleep(1000);

      // call to the swap endpoint
      const ticketId = await boundPool.swap_y({
        memeTokensOut: new BN(1),
        quoteTokensIn: new BN(50500 * 1e9),
      });

      const poolInfo = await boundPool.fetch();

      sleep(1000);

      const [amm, staking] = await boundPool.go_live();

      const solWallet = await createWrappedNativeAccount(
        provider.connection,
        payer,
        user.publicKey,
        10e9
      );

      const memeWallet = await createAssociatedTokenAccount(
        provider.connection,
        payer,
        poolInfo.memeReserve.mint,
        user.publicKey
      );

      await amm.swap(user, solWallet, memeWallet, 1e9, 1);

      await staking.add_fees(amm);
    });
  });
}
