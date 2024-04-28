  import {
    PublicKey,
    Keypair,
  } from "@solana/web3.js";
  import { memechan} from "./helpers";
  
  export interface BoundMerge {
    pool: PublicKey;
    user: Keypair;
    ticketToMerge: MemeTicket;
  }

  export interface StakingMerge {
    staking: PublicKey;
    user: Keypair;
    ticketToMerge: MemeTicket;
  }

  export class MemeTicket {

    public constructor(public id: PublicKey) {
      //
    }
  
    public async fetch() {
      return memechan.account.memeTicket.fetch(this.id);
    }

    

    public async bound_merge(
      input: BoundMerge
    ): Promise<MemeTicket> {
    
      let user = input.user;

      await memechan.methods.boundMergeTickets()
        .accounts({
          owner: user.publicKey,
          pool: input.pool,
          ticketFrom: input.ticketToMerge.id,
          ticketInto: this.id
        })
        .signers([user])
        .rpc();
        
      return this;
    }
  
    public async staking_merge(
      input: StakingMerge
    ): Promise<MemeTicket> {
    
      let user = input.user;

      await memechan.methods.stakingMergeTickets()
        .accounts({
          owner: user.publicKey,
          staking: input.staking,
          ticketFrom: input.ticketToMerge.id,
          ticketInto: this.id
        })
        .signers([user])
        .rpc();
        
      return this;
    }

  

  }
  