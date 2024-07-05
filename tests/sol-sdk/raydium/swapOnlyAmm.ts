import assert from "assert";

import { jsonInfo2PoolKeys, Liquidity, LiquidityPoolKeys, Percent, Token, TokenAmount } from "@raydium-io/raydium-sdk";
import { Connection, Keypair } from "@solana/web3.js";
import { formatAmmKeysById } from "./formatAmmKeysById";
import { makeTxVersion } from "./config";
import { MEMECHAN_QUOTE_MINT } from "../config/config";
import { buildAndSendTx, getWalletTokenAccount } from "../util";

export type WalletTokenAccounts = Awaited<ReturnType<typeof getWalletTokenAccount>>;
export type SawpOnlyAmmInputInfo = {
  outputToken: Token;
  targetPool: string;
  inputTokenAmount: TokenAmount;
  slippage: Percent;
  walletTokenAccounts: WalletTokenAccounts;
  wallet: Keypair;
  connection: Connection;
};

export async function swapOnlyAmm(input: SawpOnlyAmmInputInfo) {
  const { connection, wallet } = input;
  // -------- pre-action: get pool info --------
  const targetPoolInfo = await formatAmmKeysById(input.targetPool, connection);
  assert(targetPoolInfo, "cannot find the target pool");

  console.log("targetPoolInfo:", targetPoolInfo);

  const poolKeys = jsonInfo2PoolKeys(targetPoolInfo) as LiquidityPoolKeys;

  poolKeys.baseMint = input.outputToken.mint;
  poolKeys.quoteMint = MEMECHAN_QUOTE_MINT;

  // -------- step 1: coumpute amount out --------

  const poolInfo = await Liquidity.fetchInfo({ connection, poolKeys });

  console.log('poolInfo:', poolInfo)

  const { amountOut, minAmountOut } = Liquidity.computeAmountOut({
    poolKeys: poolKeys,
    poolInfo: poolInfo,
    amountIn: input.inputTokenAmount,
    currencyOut: input.outputToken,
    slippage: input.slippage,
  })

  console.log("amountOut:", amountOut.toFixed(), "  minAmountOut: ", minAmountOut.toFixed());

  // -------- step 2: create instructions by SDK function --------
  const { innerTransactions } = await Liquidity.makeSwapInstructionSimple({
    connection,
    poolKeys,
    userKeys: {
      tokenAccounts: input.walletTokenAccounts,
      owner: input.wallet.publicKey,
    },
    amountIn: input.inputTokenAmount,
    amountOut: minAmountOut,
    fixedSide: "in",
    makeTxVersion,
  });

  console.log("amountOut:", amountOut.toFixed(), "  minAmountOut: ", minAmountOut.toFixed());

  return { txids: await buildAndSendTx(connection, wallet, innerTransactions, { skipPreflight: true }) };
}

// async function howToUse() {
//   const inputToken = DEFAULT_TOKEN.USDC // USDC
//   const outputToken = DEFAULT_TOKEN.RAY // RAY
//   const targetPool = 'pool id' // USDC-RAY pool
//   const inputTokenAmount = new TokenAmount(inputToken, 10000)
//   const slippage = new Percent(1, 100)
//   const walletTokenAccounts = await getWalletTokenAccount(connection, wallet.publicKey)

//   swapOnlyAmm({
//     outputToken,
//     targetPool,
//     inputTokenAmount,
//     slippage,
//     walletTokenAccounts,
//     wallet: wallet,
//   }).then(({ txids }) => {
//     /** continue with txids */
//     console.log('txids', txids)
//   })
// }
