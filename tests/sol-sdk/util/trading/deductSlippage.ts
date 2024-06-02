import BigNumber from "bignumber.js";

/**
 * Deducts slippage percentage from the given output amount.
 * @param {BigNumber} amount - The output amount before deduction of slippage.
 * @param {number} slippagePercentage - The percentage of slippage to be deducted.
 * @return {BigNumber} The output amount after deduction of slippage.
 */
export function deductSlippage(amount: BigNumber, slippagePercentage: number): BigNumber {
  if (typeof slippagePercentage !== "number" || isNaN(slippagePercentage)) {
    throw new Error("Slippage percentage must be a valid number.");
  }

  if (slippagePercentage < 0 || slippagePercentage >= 100) {
    throw new Error("Slippage percentage must be between 0 (inclusive) and 100 (exclusive).");
  }

  const slippageAmount: BigNumber = amount.times(slippagePercentage / 100);

  return amount.minus(slippageAmount);
}
