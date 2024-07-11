import { Liquidity, Percent, Token, TokenAmount, jsonInfo2PoolKeys } from "@raydium-io/raydium-sdk";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Connection } from "@solana/web3.js";
import BigNumber from "bignumber.js";
import { MEMECHAN_MEME_TOKEN_DECIMALS, MEMECHAN_QUOTE_TOKEN } from "../config/config";
import { makeTxVersion } from "../raydium/config";
import { formatAmmKeysById } from "../raydium/formatAmmKeysById";
import {
  GetSwapMemeOutputArgs,
  GetSwapMemeTransactionsArgs,
  GetSwapMemeTransactionsByOutputArgs,
  SwapMemeOutput,
} from "./types";
import { getNumeratorAndDenominator } from "./utils";

export class LivePool {
  public static async getBuyMemeOutput({
    poolAddress,
    memeCoinMint,
    amountIn,
    slippagePercentage,
    connection,
  }: GetSwapMemeOutputArgs): Promise<SwapMemeOutput> {
    const quoteAmountIn = new TokenAmount(MEMECHAN_QUOTE_TOKEN, amountIn, false);
    const tokenOut = new Token(TOKEN_PROGRAM_ID, memeCoinMint, MEMECHAN_MEME_TOKEN_DECIMALS);
    const { numerator, denominator } = getNumeratorAndDenominator(slippagePercentage);
    const slippage = new Percent(numerator, denominator);

    const targetPoolInfo = await formatAmmKeysById(poolAddress, connection);

    if (!targetPoolInfo) {
      throw new Error(`[LivePool.getBuyMemeOutput] Cannot find data for pool ${poolAddress}`);
    }

    const poolKeys = jsonInfo2PoolKeys(targetPoolInfo);
    const poolInfo = await Liquidity.fetchInfo({ connection, poolKeys });

    const { minAmountOut } = Liquidity.computeAmountOut({
      poolKeys: poolKeys,
      poolInfo: poolInfo,
      amountIn: quoteAmountIn,
      currencyOut: tokenOut,
      slippage,
    });

    return { minAmountOut, poolKeys, wrappedAmountIn: quoteAmountIn };
  }

  public static async getBuyMemeTransactionsByOutput({
    wrappedAmountIn,
    connection,
    walletTokenAccounts,
    payer,
    minAmountOut,
    poolKeys,
  }: GetSwapMemeTransactionsByOutputArgs) {
    const { innerTransactions } = await Liquidity.makeSwapInstructionSimple({
      connection,
      amountIn: wrappedAmountIn,
      poolKeys,
      userKeys: {
        tokenAccounts: walletTokenAccounts,
        owner: payer,
      },
      fixedSide: "in",
      amountOut: minAmountOut,
      makeTxVersion,
    });

    return innerTransactions;
  }

  public static async getBuyMemeTransactions({
    poolAddress,
    memeCoinMint,
    amountIn,
    payer,
    slippagePercentage,
    connection,
    walletTokenAccounts,
  }: GetSwapMemeTransactionsArgs) {
    const { minAmountOut, poolKeys, wrappedAmountIn } = await LivePool.getBuyMemeOutput({
      poolAddress,
      memeCoinMint,
      amountIn,
      connection,
      slippagePercentage,
    });

    const innerTransactions = await LivePool.getBuyMemeTransactionsByOutput({
      connection,
      minAmountOut,
      payer,
      poolKeys,
      wrappedAmountIn,
      walletTokenAccounts,
    });

    return innerTransactions;
  }

  public static async getSellMemeOutput({
    poolAddress,
    memeCoinMint,
    amountIn,
    slippagePercentage,
    connection,
  }: GetSwapMemeOutputArgs): Promise<SwapMemeOutput> {
    const memeToken = new Token(TOKEN_PROGRAM_ID, memeCoinMint, MEMECHAN_MEME_TOKEN_DECIMALS);
    const memeAmountIn = new TokenAmount(memeToken, amountIn, false);
    const tokenOut = MEMECHAN_QUOTE_TOKEN;
    const { numerator, denominator } = getNumeratorAndDenominator(slippagePercentage);
    const slippage = new Percent(numerator, denominator);

    const targetPoolInfo = await formatAmmKeysById(poolAddress, connection);

    if (!targetPoolInfo) {
      throw new Error(`[LivePool.getSellMemeOutput] Cannot find data for pool ${poolAddress}`);
    }

    const poolKeys = jsonInfo2PoolKeys(targetPoolInfo);
    const poolInfo = await Liquidity.fetchInfo({ connection, poolKeys });

    const { minAmountOut } = Liquidity.computeAmountOut({
      poolKeys: poolKeys,
      poolInfo: poolInfo,
      amountIn: memeAmountIn,
      currencyOut: tokenOut,
      slippage,
    });

    return { minAmountOut, poolKeys, wrappedAmountIn: memeAmountIn };
  }

  public static async getSellMemeTransactionsByOutput({
    wrappedAmountIn,
    connection,
    walletTokenAccounts,
    payer,
    minAmountOut,
    poolKeys,
  }: GetSwapMemeTransactionsByOutputArgs) {
    const { innerTransactions } = await Liquidity.makeSwapInstructionSimple({
      connection,
      amountIn: wrappedAmountIn,
      poolKeys,
      userKeys: {
        tokenAccounts: walletTokenAccounts,
        owner: payer,
      },
      fixedSide: "in",
      amountOut: minAmountOut,
      makeTxVersion,
    });

    return innerTransactions;
  }

  public static async getSellMemeTransactions({
    poolAddress,
    memeCoinMint,
    amountIn,
    payer,
    slippagePercentage,
    connection,
    walletTokenAccounts,
  }: GetSwapMemeTransactionsArgs) {
    const { minAmountOut, poolKeys, wrappedAmountIn } = await LivePool.getSellMemeOutput({
      poolAddress,
      memeCoinMint,
      amountIn,
      connection,
      slippagePercentage,
    });

    const innerTransactions = await LivePool.getSellMemeTransactionsByOutput({
      connection,
      minAmountOut,
      payer,
      poolKeys,
      wrappedAmountIn,
      walletTokenAccounts,
    });

    return innerTransactions;
  }

  public static async getMemePrice({
    poolAddress,
    quotePriceInUsd,
    connection,
  }: {
    poolAddress: string;
    quotePriceInUsd: number;
    connection: Connection;
  }) {
    const targetPoolInfo = await formatAmmKeysById(poolAddress, connection);
    const poolKeys = jsonInfo2PoolKeys(targetPoolInfo);

    const quoteAmountIn = new TokenAmount(MEMECHAN_QUOTE_TOKEN, 10);
    const tokenOut = new Token(TOKEN_PROGRAM_ID, poolKeys.baseMint, MEMECHAN_MEME_TOKEN_DECIMALS);
    const slippage = new Percent(1, 10000);

    const { amountOut } = Liquidity.computeAmountOut({
      poolKeys: poolKeys,
      poolInfo: await Liquidity.fetchInfo({ connection, poolKeys }),
      amountIn: quoteAmountIn,
      currencyOut: tokenOut,
      slippage: slippage,
    });

    const memePriceInQuote = new BigNumber(10).div(amountOut.toExact()).toString();
    const memePriceInUsd = new BigNumber(memePriceInQuote).multipliedBy(quotePriceInUsd).toString();

    return { priceInQuote: memePriceInQuote, priceInUsd: memePriceInUsd };
  }
}
