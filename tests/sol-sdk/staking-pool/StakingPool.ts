import { Program } from "@coral-xyz/anchor";
import { AccountMeta, PublicKey } from "@solana/web3.js";
import { MemechanClient } from "../MemechanClient";
import { BoundPoolClient } from "../bound-pool/BoundPool";
import { MemeTicket, MemeTicketFields } from "../memeticket/MemeTicket";
import { MemechanSol } from "../../../target/types/memechan_sol";

export class StakingPool {
  constructor(
    public id: PublicKey,
    private client: MemechanClient,
    public pool: PublicKey,
    public memeVault: PublicKey,
    public memeMint: PublicKey,
    public quote_vault: PublicKey,
    public amm: PublicKey
  ) {}

  public static async fromStakingPoolId({
    client,
    poolAccountAddressId,
  }: {
    client: MemechanClient;
    poolAccountAddressId: PublicKey;
  }) {
    const stakingPoolObjectData =
      await client.memechanProgram.account.stakingPool.fetch(
        poolAccountAddressId
      );

    // console.log("stakingPoolObjectData:", stakingPoolObjectData);

    const boundClientInstance = new StakingPool(
      poolAccountAddressId,
      client,
      stakingPoolObjectData.pool,
      stakingPoolObjectData.memeVault,
      stakingPoolObjectData.memeMint,
      stakingPoolObjectData.quoteVault,
      stakingPoolObjectData.chanAmmPool
    );

    return boundClientInstance;
  }

  public static findSignerPda(
    publicKey: PublicKey,
    memechanProgramId: PublicKey
  ): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("staking"), publicKey.toBytes()],
      memechanProgramId
    )[0];
  }

  public async getHoldersCount() {
    return StakingPool.getHoldersCount(this.pool, this.memeMint, this.client);
  }

  public async getHoldersList() {
    return StakingPool.getHoldersList(this.pool, this.memeMint, this.client);
  }

  /**
   * Fetches all tickets for corresponding pool id
   */
  public async fetchRelatedTickets(
    pool = this.pool,
    client = this.client
  ): Promise<MemeTicketFields[]> {
    return MemeTicket.fetchRelatedTickets(pool, client);
  }

  /**
   * Fetches all unique token holders and memetickets owners for pool; then returns their number
   */
  public static async getHoldersCount(
    pool: PublicKey,
    mint: PublicKey,
    client: MemechanClient
  ) {
    return (await StakingPool.getHoldersList(pool, mint, client)).length;
  }

  /**
   * Fetches all unique token holders and memetickets owners for pool; then returns thier addresses
   */
  public static async getHoldersList(
    pool: PublicKey,
    mint: PublicKey,
    client: MemechanClient
  ) {
    const ticketHolderList = await BoundPoolClient.getHoldersList(pool, client);
    const tokenHolderList = await StakingPool.getTokenHolderListHelius(
      mint,
      client.heliusApiUrl
    );

    return Array.from(tokenHolderList);
  }

  public static async getTokenHolderListHelius(mint: PublicKey, url: string) {
    return [];
  }

  private async fetch(program = this.client.memechanProgram) {
    return program.account.stakingPool.fetch(this.id);
  }

  public static async all(program: Program<MemechanSol>) {
    return program.account.stakingPool.all();
  }

  public findSignerPda(): PublicKey {
    return StakingPool.findSignerPda(
      this.id,
      this.client.memechanProgram.programId
    );
  }

  private getAccountMeta(pubkey: PublicKey): AccountMeta {
    return { isSigner: false, isWritable: true, pubkey };
  }
}
