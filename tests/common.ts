import { PublicKey } from "@solana/web3.js";
import { MemechanClient } from "./sol-sdk/MemechanClient";
import { ADMIN_PUB_KEY } from "./sol-sdk/config/config";

//export const connection = new Connection(RPC_API_CLUSTER);
export const admin = ADMIN_PUB_KEY;
export const client = new MemechanClient({});

export const BP_FEE_VAULT_OWNER = new PublicKey(
  "6YNJG9KDex3eNAmh1i64KUDbfKBiESkew3AWmnf6FiCy"
);
export const SWAP_FEE_VAULT_OWNER = new PublicKey(
  "xqzvZzKFCjvPuRqkyg5rxA95avrvJxesZ41rCLfYwUM"
);
export const LP_FEE_VAULT_OWNER = new PublicKey(
  "HQ1wVLaBcnuoUozegyX7r45yn6ogHvQjdPNj53iweC5V"
);

export const BE_AUTH = new PublicKey(
  "8JvLLwD7oBvPfg3NL1dAL7GbQJuJznP4MhsYnfNkKjAR"
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
