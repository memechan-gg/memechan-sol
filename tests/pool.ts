import {
  PublicKey,
  Keypair,
} from "@solana/web3.js";
import { amm, provider} from "./helpers";
import { getAccount } from "@solana/spl-token";
import BN from "bn.js";
import { discountAddress } from "./amm";

export class AmmPool {
  public constructor(public id: PublicKey, public tollAuthority: PublicKey) {
    //
  }

  public async fetch() {
    return amm.account.pool.fetch(this.id);
  }

  public static signerFrom(publicKey: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("signer"), publicKey.toBytes()],
      amm.programId
    )[0];
  }

  public async swap(
    user: Keypair,
    sellWallet: PublicKey,
    buyWallet: PublicKey,
    sell: number,
    minBuy: number
  ) {
    const pool = await this.fetch();
    const getVaultOfWallet = async (wallet: PublicKey) => {
      const { mint } = await getAccount(provider.connection, wallet);
      const reserves = pool.reserves as any[];
      return reserves.find((r) => r.mint.toBase58() === mint.toBase58()).vault;
    };

    await amm.methods
      .swap({ amount: new BN(sell) }, { amount: new BN(minBuy) })
      .accounts({
        user: user.publicKey,
        discount: discountAddress(user.publicKey),
        sellWallet,
        sellVault: await getVaultOfWallet(sellWallet),
        buyWallet,
        buyVault: await getVaultOfWallet(buyWallet),
        pool: this.id,
        poolSigner: this.signerPda(),
        programTollWallet: pool.programTollWallet,
        lpMint: pool.mint,
      })
      .signers([user])
      .rpc();
  }

  public signer(): PublicKey {
    return AmmPool.signerFrom(this.id);
  }

  public signerPda(): PublicKey {
    return AmmPool.signerFrom(this.id);
  }
}
