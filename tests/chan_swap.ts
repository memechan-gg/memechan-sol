import BN from "bn.js";
import {
  QUOTE_MINT,
  adminSigner,
  airdrop,
  mintChan,
  mintQuote,
  payer,
} from "./helpers";
import { TargetConfig } from "./sol-sdk/targetconfig/TargetConfig";
import { client } from "./common";
import {
  CHAN_TOKEN_INFO,
  MEMECHAN_QUOTE_TOKEN,
  memechan,
} from "./sol-sdk/config/config";
import { PublicKey } from "@saberhq/solana-contrib";
import { SystemProgram } from "@solana/web3.js";
import {} from "@solana/spl-token";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

export class ChanSwapWrapper {
  static chanSwapId(): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("chan_swap")],
      memechan.programId
    )[0];
  }

  static chanSwapSigner(): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("chan_swap_signer")],
      memechan.programId
    )[0];
  }

  static async new(num: number = 1, denom: number = 1000) {
    const tcdata = await memechan.account.chanSwap.fetchNullable(
      ChanSwapWrapper.chanSwapId()
    );

    if (tcdata !== null) {
      return;
    }

    await airdrop(adminSigner.publicKey);

    const chanVault = await getOrCreateAssociatedTokenAccount(
      client.connection,
      payer,
      new PublicKey(CHAN_TOKEN_INFO.address),
      ChanSwapWrapper.chanSwapSigner(),
      true
    );

    console.log(chanVault);

    await mintChan(chanVault.address, 10_000_000 * 10 ** 9);

    await memechan.methods
      .newChanSwap(new BN(num), new BN(denom))
      .accounts({
        chanSwap: ChanSwapWrapper.chanSwapId(),
        chanSwapSignerPda: ChanSwapWrapper.chanSwapSigner(),
        chanVault: chanVault.address,
        sender: adminSigner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([adminSigner])
      .rpc({ skipPreflight: true });
  }
}
