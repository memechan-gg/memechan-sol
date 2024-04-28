  import {
    PublicKey,
    Keypair,
  } from "@solana/web3.js";
  import { memechan} from "./helpers";
  
  export interface BoundMerge {
    pool: PublicKey;
    user: Keypair;
    ticketOne: MemeTicket;
    ticketTwo: MemeTicket;
    poolSignerPda: PublicKey;
  }

  export interface StakingMerge {
    staking: PublicKey;
    user: Keypair;
    stakingSignerPda: PublicKey;
  }

  export class MemeTicket {

    public constructor(public id: PublicKey) {
      //
    }
  
    public async fetch() {
      return memechan.account.memeTicket.fetch(this.id);
    }

    // public async bound_merge(
    //     input: BoundMerge
    //   ): Promise<MemeTicket> {
    
    //     const pool = input.pool;
    //     const poolSignerPda = input.poolSignerPda;
    
    //     //memechan.methods.mergeTickets();
        
    //     return pool;
    //   }
  

  

  }
  