import { IdlAccounts } from "@coral-xyz/anchor";
import { ParsedTransactionWithMeta, PublicKey } from "@solana/web3.js";
import { MemechanClient } from "../../MemechanClient";
import { MemechanSol } from "../../schema/types/memechan_sol";

export type StakingPool = IdlAccounts<MemechanSol>["stakingPool"];

export type InitStakingPoolInstructionParsed = {
  poolAddr: PublicKey;
  stakingAddr: PublicKey;
  staking: StakingPool;
  type: "init_staking";
};

export async function ParseInitStakingInstruction(
  tx: ParsedTransactionWithMeta,
  index: number,
  client: MemechanClient,
): Promise<InitStakingPoolInstructionParsed | undefined> {
  const ix = tx.transaction.message.instructions[index];

  if (!("accounts" in ix)) {
    return undefined;
  }

  const poolAddr = ix.accounts[1];
  const stakingAddr = ix.accounts[8];
  const staking = await client.memechanProgram.account.stakingPool.fetch(stakingAddr);

  const ispParsed: InitStakingPoolInstructionParsed = {
    poolAddr,
    staking,
    stakingAddr,
    type: "init_staking",
  };

  return ispParsed;
}
