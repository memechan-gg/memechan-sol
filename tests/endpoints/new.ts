import { expect } from "chai";
import { QUOTE_MINT, airdrop, provider } from "../helpers";
import { client } from "../common";
import { Keypair } from "@solana/web3.js";
import { Token } from "@raydium-io/raydium-sdk";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BoundPoolWrapper } from "../bound_pool";
import { TargetConfig } from "../sol-sdk/targetconfig/TargetConfig";
import { TargetConfigWrapper } from "../target_config";
import { before, beforeEach } from "mocha";

export function test() {
  describe("create_bound_pool", () => {
    it("creates target config", async () => {
      await TargetConfigWrapper.new();
    });

    it("creates bound pool", async () => {
      const user = Keypair.generate();
      await airdrop(user.publicKey);

      const boundPool = await BoundPoolWrapper.new();
      const info = await boundPool.fetch();
      console.log(info);
    });
  });
}
