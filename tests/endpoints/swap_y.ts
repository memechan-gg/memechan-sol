import { expect } from "chai";
import { BoundPool } from "../bound_pool";
import { AccountMeta, Keypair, PublicKey } from "@solana/web3.js";
import { createAccount, getAccount } from "@solana/spl-token";
import { memechan, payer, provider, sleep } from "../helpers";
import { BN } from "@project-serum/anchor";

export function test() {
  describe("swap_y", () => {
    const user = Keypair.generate();

    it("swaps sol->memecoin", async () => {
      const pool = await BoundPool.new();

      const info = await pool.fetch();

      const mint1 = info.memeMint;
      const mint2 = info.solReserve.mint;

      const userTokenWallet1 = await createAccount(
        provider.connection,
        payer,
        mint1,
        user.publicKey
      );

      const userTokenWallet2 = await createAccount(
        provider.connection,
        payer,
        mint2,
        user.publicKey
      );

      //BoundPool.airdropLiquidityTokens(mint1, userTokenWallet1, pool.id, 1_000_000);
      //BoundPool.airdropLiquidityTokens(mint2, userTokenWallet2, pool.id, 1_000_000);

      await sleep(1000);


      // call to the depositLiquidity endpoint
      const ticketId = await pool.swap_y({
        maxAmountTokens: new BN(0),
      });

      sleep(1000);

      const poolInfo = await pool.fetch();
      console.log("--pool--")
      console.log(poolInfo.adminFeesMeme.toString())
      console.log(poolInfo.adminFeesSol.toString())
      console.log(poolInfo.memeAmt.toString())
      console.log(poolInfo.solReserve.tokens.toString())
      console.log(poolInfo.locked)


      const ticketInfo = await memechan.account.memeTicket.fetch(ticketId);
      console.log(ticketInfo.amount.toString())
      console.log(ticketInfo.owner.toString())
      console.log(ticketInfo.pool.toString())


      // const poolTokenVaultInfo1 = await getAccount(
      //   provider.connection,
      //   info.reserves[0].vault
      // );
      // const userTokenWalletInfo1 = await getAccount(
      //   provider.connection,
      //   userTokenWallet1
      // );

      // const poolTokenVaultInfo2 = await getAccount(
      //   provider.connection,
      //   info.reserves[1].vault
      // );
      // const userTokenWalletInfo2 = await getAccount(
      //   provider.connection,
      //   userTokenWallet2
      // );

      // const lpTokenWalletAccountInfo = await getAccount(
      //   provider.connection,
      //   lpTokenWallet
      // );

      // const poolTokenVaultAmount1 = poolTokenVaultInfo1.amount;
      // const userTokenWalletAmount1 = userTokenWalletInfo1.amount;

      // const poolTokenVaultAmount2 = poolTokenVaultInfo2.amount;
      // const userTokenWalletAmount2 = userTokenWalletInfo2.amount;

      // const lpTokenAmount = lpTokenWalletAccountInfo.amount;

      // expect(poolTokenVaultAmount1).to.be.eq(BigInt(100));
      // expect(poolTokenVaultAmount2).to.be.eq(BigInt(10));

      // expect(userTokenWalletAmount1).to.be.eq(BigInt(1_000_000 - 100));
      // expect(userTokenWalletAmount2).to.be.eq(BigInt(1_000_000 - 10));

      // // since this is the first deposit, we expect that the total amount
      // // of lp tokens minted corresponds to the minimum value of deposited tokens
      // expect(lpTokenAmount).to.be.eq(BigInt(10));

      // // test that nothing else changed in the pool
      // expect(info.reserves[0].tokens.amount.toNumber() + 100).to.be.deep.eq(
      //   poolInfo.reserves[0].tokens.amount.toNumber()
      // );
      // expect(info.reserves[1].tokens.amount.toNumber() + 10).to.be.deep.eq(
      //   poolInfo.reserves[1].tokens.amount.toNumber()
      // );

      // // nothing else changes in the pool
      // delete info.reserves;
      // delete poolInfo.reserves;

      // expect(info).to.be.deep.eq(poolInfo);

      // await pool.depositLiquidity({
      //   maxAmountTokens,
      //   vaultsAndWallets,
      //   lpTokenWallet,
      //   user,
      // });

      // const newDepositVaultAmount1 = (
      //   await getAccount(provider.connection, poolTokenVaultInfo1.address)
      // ).amount;

      // const newDepositVaultAmount2 = (
      //   await getAccount(provider.connection, poolTokenVaultInfo2.address)
      // ).amount;

      // expect(newDepositVaultAmount1).to.be.deep.eq(BigInt(200));
      // expect(newDepositVaultAmount2).to.be.deep.eq(BigInt(20));

      // const newUserWalletAmount1 = (
      //   await getAccount(provider.connection, userTokenWallet1)
      // ).amount;

      // const newUserWalletAmount2 = (
      //   await getAccount(provider.connection, userTokenWallet2)
      // ).amount;

      // expect(newUserWalletAmount1).to.be.deep.eq(BigInt(999800));
      // expect(newUserWalletAmount2).to.be.deep.eq(BigInt(999980));

      // const lpTokenWalletAmount = (
      //   await getAccount(provider.connection, lpTokenWallet)
      // ).amount;

      // expect(lpTokenWalletAmount).to.be.deep.eq(BigInt(20));
    });

  });
}
