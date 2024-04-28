import { assert, expect } from "chai";
import { BoundPool } from "../bound_pool";
import { AccountMeta, Keypair, PublicKey } from "@solana/web3.js";
import { createAccount, createWrappedNativeAccount, getAccount } from "@solana/spl-token";
import { memechan, payer, provider, sleep } from "../helpers";
import { BN } from "@project-serum/anchor";

export function test() {
    describe("go_live", () => {

        it("full swap then go live", async () => {
            const pool = await BoundPool.new();

            await sleep(1000);

            const ticketId = await pool.swap_y({
                memeTokensOut: new BN(1),
                solAmountIn: new BN(303 * 1e9),
            });

            await pool.go_live({})

        });
    });

}
