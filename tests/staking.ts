import {
  TOKEN_PROGRAM_ID,
  getAccount,
} from "@solana/spl-token";
import {
  PublicKey,
} from "@solana/web3.js";
import { memechan, payer, amm, provider } from "./helpers";
import { AmmPool } from "./pool";

export class Staking {
  public constructor(public id: PublicKey) {
    //
  }


  public async fetch() {
    return memechan.account.stakingPool.fetch(this.id);
  }

  public static signerFrom(publicKey: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("staking"), publicKey.toBytes()],
      memechan.programId
    )[0];
  }

  public async add_fees(ammPool: AmmPool) {

    const info = await this.fetch();

    const pda = this.signerPda()
    const staking = this.id;

    const ammInfo = await ammPool.fetch()

    const getAccountMetaFromPublicKey = (pk) => {
      return { isSigner: false, isWritable: true, pubkey: pk };
    };

    await memechan.methods.addFees()
      .accounts({
        memeVault: info.memeVault,
        wsolVault: info.wsolVault,
        staking,
        aldrinPoolAcc: ammPool.id,
        aldrinAmmProgram: amm.programId,
        aldrinLpMint: ammInfo.mint,
        aldrinPoolLpWallet: ammInfo.programTollWallet,
        aldrinPoolSigner: ammPool.signer(),
        stakingPoolSignerPda: this.signer(),
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .remainingAccounts([
        getAccountMetaFromPublicKey(ammInfo.reserves[0].vault),
        getAccountMetaFromPublicKey(ammInfo.reserves[1].vault)
      ])
      .signers([payer])
      .rpc();
  }

  public signer(): PublicKey {
    return Staking.signerFrom(this.id);
  }

  public signerPda(): PublicKey {
    return Staking.signerFrom(this.id);
  }
}
