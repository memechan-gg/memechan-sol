import { Wallet } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";

export const NoWalletAdapter: Wallet = (() => {
  const keypair = Keypair.generate();

  const nodeWalletAdapterObj = {
    publicKey: keypair.publicKey,
    payer: keypair,
    signTransaction: () => {
      throw new Error("NoWalletAdapter");
    },
    signAllTransactions: () => {
      throw new Error("NoWalletAdapter");
    },
  };

  return nodeWalletAdapterObj;
})();
