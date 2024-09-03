import {
  AnchorProvider,
  Program,
  setProvider,
  web3,
  workspace,
} from "@project-serum/anchor";
import { MemechanSol } from "../target/types/memechan_sol";
import { Keypair, PublicKey } from "@solana/web3.js";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import {
  createAssociatedTokenAccountIdempotentInstruction,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

process.env.ANCHOR_WALLET = process.env.HOME + "/.config/solana/id.json";
const wallet = NodeWallet.local();
const payer = wallet.payer;

const provider = new AnchorProvider(
  new web3.Connection("http://localhost:8899"),
  wallet,
  { commitment: "confirmed" }
);

setProvider(provider);

const memechan = workspace.MemechanSol as Program<MemechanSol>;

import "dotenv/config";

//yarn tsx scripts/recover-lp.ts
async function recoverLp() {
  console.log(`sending lp tokens to ${payer.publicKey}`);
  const staking = new PublicKey("4HztW6sS78Eyq7wskcAkd8XXCPfwbFgR9FsK7HuTBY8W");
  console.log(await memechan.account.stakingPool.fetch(staking));
  const lp_ata = new PublicKey("4y25GAmFGZQdBFV54jZwZ9jzcYYCCkMpfTsG2T4mAoxG");
  const memeMint = new PublicKey(
    "35fN6LMYt6cKsemgbR28nFooiJtcnvaKPCeRXyuMKfoF"
  );
  const lpMint = new PublicKey("C3fPoi6qNWsKZA1DKVh8mTv66g5P9Jwo4ciXHsJmnJ4s");
  const adminSigner = Keypair.fromSecretKey(
    Buffer.from(JSON.parse(process.env.ADMIN_PROD_KEY))
  );

  const userAta = getAssociatedTokenAddressSync(lpMint, payer.publicKey, false);

  const cix = createAssociatedTokenAccountIdempotentInstruction(
    adminSigner.publicKey,
    userAta,
    payer.publicKey,
    lpMint
  );

  const stakingPoolSignerPda = getStakingSignerPDA(staking);
  console.log(1);

  const txDig = await memechan.methods
    .recoverLp()
    .accounts({
      memeMint,
      poolLpTokenAta: lp_ata,
      userDestinationLpTokenAta: userAta,
      signer: adminSigner.publicKey,
      staking,
      stakingPoolSignerPda,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([adminSigner])
    .preInstructions([cix])
    .rpc({ skipPreflight: true });
  console.log(`got transaction digest ${txDig}`);
}

function getStakingSignerPDA(staking: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("staking"), staking.toBytes()],
    memechan.programId
  )[0];
}

recoverLp();
