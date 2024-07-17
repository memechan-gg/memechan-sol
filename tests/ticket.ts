import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { memechan } from "./sol-sdk/config/config";

export interface BoundMerge {
  pool: PublicKey;
  user: Keypair;
  ticketToMerge: MemeTicketWrapper;
}

export interface StakingMerge {
  staking: PublicKey;
  user: Keypair;
  ticketToMerge: MemeTicketWrapper;
}

export interface CloseArgs {
  user: Keypair;
}

export class MemeTicketWrapper {
  public constructor(public id: PublicKey) {
    //
  }

  public async fetch() {
    return memechan.account.memeTicket.fetch(this.id);
  }

  public async bound_merge(input: BoundMerge): Promise<MemeTicketWrapper> {
    let user = input.user;

    await memechan.methods
      .boundMergeTickets()
      .accounts({
        owner: user.publicKey,
        pool: input.pool,
        ticketFrom: input.ticketToMerge.id,
        ticketInto: this.id,
      })
      .signers([user])
      .rpc();

    return this;
  }

  public async staking_merge(input: StakingMerge): Promise<MemeTicketWrapper> {
    let user = input.user;

    await memechan.methods
      .stakingMergeTickets()
      .accounts({
        owner: user.publicKey,
        staking: input.staking,
        ticketFrom: input.ticketToMerge.id,
        ticketInto: this.id,
      })
      .signers([user])
      .rpc();

    return this;
  }

  public async close(input: CloseArgs): Promise<MemeTicketWrapper> {
    let user = input.user;

    await memechan.methods
      .closeTicket()
      .accounts({
        owner: user.publicKey,
        ticket: this.id,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    return this;
  }
}
