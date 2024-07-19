import { Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { MemeTicket } from "./MemeTicket";

export interface BoundMerge {
  pool: PublicKey;
  user: Keypair;
  ticketsToMerge: MemeTicket[];
}

export type GetBoundMergeTransactionArgs = BoundMerge & {
  transaction?: Transaction;
};

export interface StakingMerge {
  staking: PublicKey;
  user: Keypair;
  ticketsToMerge: MemeTicket[];
}

export type GetStakingMergeTransactionArgs = StakingMerge & {
  transaction?: Transaction;
};

export interface CloseArgs {
  user: Keypair;
}

export type GetCloseTransactionArgs = CloseArgs & { transaction?: Transaction };
