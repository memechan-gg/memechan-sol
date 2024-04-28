import {
    createAccount,
    createMint,
    getAccount,
    mintTo,
    TOKEN_PROGRAM_ID,
  } from "@solana/spl-token";
  import {
    AccountMeta,
    PublicKey,
    Keypair,
    Signer,
    SystemProgram,
  } from "@solana/web3.js";
  import { airdrop, memechan, payer, provider } from "./helpers";
  import { BN } from "@project-serum/anchor";
  
  export class Staking {
    private constructor(public id: Keypair) {
      //
    }
  
  
    public async fetch() {
      return memechan.account.stakingPool.fetch(this.id.publicKey);
    }
  
    public static signerFrom(publicKey: PublicKey): PublicKey {
      return PublicKey.findProgramAddressSync(
        [Buffer.from("staking"), publicKey.toBytes()],
        memechan.programId
      )[0];
    }
  
    public signer(): PublicKey {
      return Staking.signerFrom(this.id.publicKey);
    }
  
    public signerPda(): PublicKey {
      return Staking.signerFrom(this.id.publicKey);
    }
  }
  