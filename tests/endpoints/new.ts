import {
  LUTSLOT,
  QUOTE_MINT,
  admin,
  adminSigner,
  airdrop,
  getLUTPDA,
  mintKeypair,
  payer,
  provider,
  sleep,
} from "../helpers";
import {
  Keypair,
  AddressLookupTableProgram,
  PublicKey,
  Transaction,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BoundPoolWrapper } from "../bound_pool";
import { TargetConfig } from "../sol-sdk/targetconfig/TargetConfig";
import { TargetConfigWrapper } from "../target_config";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import { ChanSwapWrapper } from "../chan_swap";
import { DEFAULT_TARGET, memechan } from "../sol-sdk/config/config";
import { createMint } from "@solana/spl-token";

export function test() {
  describe("create_bound_pool", () => {
    it("creates target config", async () => {
      await TargetConfigWrapper.new(DEFAULT_TARGET);
      await airdrop(admin);

      await createMint(provider.connection, payer, admin, null, 9, mintKeypair);
    });

    it("creates chan swap", async () => {
      await ChanSwapWrapper.new();
    });

    it("create LUT", async () => {
      //const slot = await client.connection.getSlot();
      const [createLUTix, LUTaddr] =
        AddressLookupTableProgram.createLookupTable({
          authority: admin,
          payer: admin,
          recentSlot: LUTSLOT,
        });
      const extendIxs = AddressLookupTableProgram.extendLookupTable({
        payer: admin,
        lookupTable: LUTaddr,
        authority: admin,
        addresses: [
          admin,
          SystemProgram.programId,
          TOKEN_PROGRAM_ID,
          ASSOCIATED_PROGRAM_ID,
          new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
          memechan.programId,
          TargetConfig.findTargetConfigPda(QUOTE_MINT, memechan.programId),
          SYSVAR_RENT_PUBKEY,
          QUOTE_MINT,
        ],
      });

      const tx = new Transaction().add(createLUTix, extendIxs);
      const txDig = await provider.connection.sendTransaction(tx, [
        adminSigner,
      ]);
      // console.debug(
      //   LUTaddr,
      //   getLUTPDA({
      //     authority: admin,
      //     recentSlot: LUTSLOT,
      //   })
      // );
    });

    it("creates bound pool", async () => {
      await sleep(2000);
      const user = Keypair.generate();
      await airdrop(user.publicKey);

      const boundPool = await BoundPoolWrapper.new();
      await sleep(100);
      // console.log(
      //   await memechan.account.boundPool.fetch(boundPool.bpClient.id)
      // );
      const info = await boundPool.fetch();
      //console.log(info);
    });

    it("creates bound pool with airdrop", async () => {
      const user = Keypair.generate();
      await airdrop(user.publicKey);

      const boundPool = await BoundPoolWrapper.new(100, true);
      await sleep(100);
      // console.log(
      //   await memechan.account.boundPool.fetch(boundPool.bpClient.id)
      // );
      const info = await boundPool.fetch();
      //console.log(info);
    });
  });
}
