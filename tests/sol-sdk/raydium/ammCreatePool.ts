import { Liquidity, Token } from "@raydium-io/raydium-sdk";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";

import { PROGRAMIDS, makeTxVersion } from "./config";
import { buildAndSendTx, getWalletTokenAccount } from "../util";
import { BN as AnchorBN } from "@coral-xyz/anchor";

const ZERO = new AnchorBN(0);
type BN = typeof ZERO;

type CalcStartPrice = {
  addBaseAmount: BN;
  addQuoteAmount: BN;
};

// function calcMarketStartPrice(input: CalcStartPrice) {
//   return input.addBaseAmount.toNumber() / 10 ** 6 / (input.addQuoteAmount.toNumber() / 10 ** 6);
// }

type LiquidityPairTargetInfo = {
  baseToken: Token;
  quoteToken: Token;
  targetMarketId: PublicKey;
};

function getMarketAssociatedPoolKeys(input: LiquidityPairTargetInfo) {
  return Liquidity.getAssociatedPoolKeys({
    version: 4,
    marketVersion: 3,
    baseMint: input.baseToken.mint,
    quoteMint: input.quoteToken.mint,
    baseDecimals: input.baseToken.decimals,
    quoteDecimals: input.quoteToken.decimals,
    marketId: input.targetMarketId,
    programId: PROGRAMIDS.AmmV4,
    marketProgramId: PROGRAMIDS.OPENBOOK_MARKET,
  });
}

type WalletTokenAccounts = Awaited<ReturnType<typeof getWalletTokenAccount>>;
type TestTxInputInfo = LiquidityPairTargetInfo &
  CalcStartPrice & {
    startTime: number; // seconds
    walletTokenAccounts: WalletTokenAccounts;
    wallet: Keypair;
    connection: Connection;
  } & {
    feeDestinationWalletAddress: string;
  };

export async function ammCreatePool(input: TestTxInputInfo) {
  // -------- step 1: make instructions --------
  const initPoolInstructionResponse = await Liquidity.makeCreatePoolV4InstructionV2Simple({
    connection: input.connection,
    programId: PROGRAMIDS.AmmV4,
    marketInfo: {
      marketId: input.targetMarketId,
      programId: PROGRAMIDS.OPENBOOK_MARKET,
    },
    baseMintInfo: input.baseToken,
    quoteMintInfo: input.quoteToken,
    baseAmount: input.addBaseAmount,
    quoteAmount: input.addQuoteAmount,
    startTime: new AnchorBN(Math.floor(input.startTime)),
    ownerInfo: {
      feePayer: input.wallet.publicKey,
      wallet: input.wallet.publicKey,
      tokenAccounts: input.walletTokenAccounts,
      useSOLBalance: true,
    },
    associatedOnly: false,
    checkCreateATAOwner: true,
    makeTxVersion,
    //feeDestinationId: new PublicKey('7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5'), // only mainnet use this
    feeDestinationId: new PublicKey(input.feeDestinationWalletAddress),
  });

  const poolInfo = getMarketAssociatedPoolKeys(input);

  return {
    txids: await buildAndSendTx(input.connection, input.wallet, initPoolInstructionResponse.innerTransactions, {
      skipPreflight: true,
    }),
    ammPool: initPoolInstructionResponse.address,
    poolInfo,
  };
}

// async function howToUse() {
//   const baseToken = DEFAULT_TOKEN.USDC // USDC
//   const quoteToken = DEFAULT_TOKEN.RAY // RAY
//   const targetMarketId = Keypair.generate().publicKey
//   const addBaseAmount = new BN(10000) // 10000 / 10 ** 6,
//   const addQuoteAmount = new BN(10000) // 10000 / 10 ** 6,
//   const startTime = Math.floor(Date.now() / 1000) + 60 * 60 * 24 * 7 // start from 7 days later
//   const walletTokenAccounts = await getWalletTokenAccount(connection, wallet.publicKey)

//   /* do something with start price if needed */
//   const startPrice = calcMarketStartPrice({ addBaseAmount, addQuoteAmount })

//   /* do something with market associated pool keys if needed */
//   const associatedPoolKeys = getMarketAssociatedPoolKeys({
//     baseToken,
//     quoteToken,
//     targetMarketId,
//   })

//   ammCreatePool({
//     startTime,
//     addBaseAmount,
//     addQuoteAmount,
//     baseToken,
//     quoteToken,
//     targetMarketId,
//     wallet,
//     walletTokenAccounts,
//   }).then(({ txids }) => {
//     /** continue with txids */
//     console.log('txids', txids)
//   })
// }
