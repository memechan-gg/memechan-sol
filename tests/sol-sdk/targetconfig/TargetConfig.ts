import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { MemechanClient } from "../MemechanClient";
import { CreateTargetConfigArgs } from "./types";
import BN from "bn.js";
import { memechan } from "../../helpers";

export class TargetConfig {
  public constructor(
    public id: PublicKey,
    public client: MemechanClient,
    public tokenMint: PublicKey,
    public tokenTargetAmount: BN
  ) {
    //
  }

  public static async fromTargetConfigId({
    client,
    accountAddressId,
  }: {
    client: MemechanClient;
    accountAddressId: PublicKey;
  }) {
    const objectData = await TargetConfig.fetch(
      client.connection,
      accountAddressId
    );

    console.log("objectData:", objectData);

    const instance = new TargetConfig(
      accountAddressId,
      client,
      objectData.tokenMint,
      objectData.tokenTargetAmount
    );

    return instance;
  }

  // public async fetch(program = this.client.memechanProgram) {
  //   return program.account.targetConfig.fetch(this.id, "confirmed");
  // }

  /**
   * Fetches the bound pool account information.
   *
   * @param {Connection} connection - The Solana RPC connection.
   * @param {PublicKey} accountId - The ID of the account to fetch.
   * @returns {Promise<T>} - The account information.
   */
  static async fetch(connection: Connection, accountId: PublicKey) {
    const accountInfo = await memechan.account.targetConfig.fetch(accountId);

    if (!accountInfo) {
      throw new Error(
        `[TargetConfig.fetch] No account info found for ${accountId}`
      );
    }

    return accountInfo;
  }

  public static async new(input: CreateTargetConfigArgs) {
    const pda = this.findTargetConfigPda(
      input.mint,
      input.client.memechanProgram.programId
    );

    const result = await input.client.memechanProgram.methods
      .newTargetConfig(input.targetAmount)
      .accounts({
        mint: input.mint,
        sender: input.payer.publicKey,
        targetConfig: pda,
        systemProgram: SystemProgram.programId,
      })
      .signers([input.payer])
      .rpc({ skipPreflight: true });

    console.log("newTargetConfig result", result);

    return new TargetConfig(pda, input.client, input.mint, input.targetAmount);
  }

  public static findTargetConfigPda(
    quoteMintPubkey: PublicKey,
    memechanProgramId: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("config"), quoteMintPubkey.toBytes()],
      memechanProgramId
    )[0];
  }

  public async changeTargetConfig(
    targetAmount: BN,
    payer: Keypair
  ): Promise<string> {
    const result = await this.client.memechanProgram.methods
      .changeTargetConfig(targetAmount)
      .accounts({
        sender: payer.publicKey,
        targetConfig: this.id,
      })
      .signers([payer])
      .rpc({ commitment: "confirmed" });

    console.log("changeTargetConfig result", result);
    return result;
  }
}
