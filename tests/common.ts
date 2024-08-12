import { Wallet } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import {
  IS_TEST_ENV,
  RPC_API_CLUSTER,
  TEST_USER_SECRET_KEY,
  WSS_API_CLUSTER,
} from "./env";
import { MemechanClient } from "./sol-sdk/MemechanClient";
import { ADMIN_PUB_KEY, memechan } from "./sol-sdk/config/config";

//export const connection = new Connection(RPC_API_CLUSTER);
export const admin = ADMIN_PUB_KEY;
export const payer = Keypair.fromSecretKey(
  Buffer.from(JSON.parse(TEST_USER_SECRET_KEY))
);
export const wallet = new Wallet(payer);
export const client = new MemechanClient({
  wallet,
  heliusApiUrl: "HELIUS_API_URL",
  rpcApiUrl: RPC_API_CLUSTER,
  wssApiUrl: WSS_API_CLUSTER,
  isTest: IS_TEST_ENV,
});

export const pointsMint = new PublicKey(
  "ptsVM2dwpBVhu6uR3D1zzoRSjm1TC8gdmBEk8jpTP1P"
);
export const pointsPda = PublicKey.findProgramAddressSync(
  [Buffer.from("points_pda")],
  memechan.programId
)[0];

export const BP_FEE_VAULT_OWNER = new PublicKey(
  "6YNJG9KDex3eNAmh1i64KUDbfKBiESkew3AWmnf6FiCy"
);
export const SWAP_FEE_VAULT_OWNER = new PublicKey(
  "xqzvZzKFCjvPuRqkyg5rxA95avrvJxesZ41rCLfYwUM"
);
export const LP_FEE_VAULT_OWNER = new PublicKey(
  "HQ1wVLaBcnuoUozegyX7r45yn6ogHvQjdPNj53iweC5V"
);

export const DUMMY_TOKEN_METADATA = {
  name: "Best Token Ever",
  symbol: "BTE",
  image:
    "https://cf-ipfs.com/ipfs/QmVevMfxFpfgBu5kHuYUPmDMaV6pWkAn3zw5XaCXxKdaBh",
  description: "This is the best token ever",
  twitter: "https://twitter.com/BestTokenEver",
  telegram: "https://t.me/BestTokenEver",
  website: "https://besttokenever.com",
  discord: "",
};
