import BigNumber from "bignumber.js";

/**
 * Removes the decimal part of a BigNumber if present.
 * @param {BigNumber} input - The input BigNumber representing a number.
 * @return {BigNumber} - The BigNumber without the decimal part.
 * @throws {Error} - Throws an error if the input is not in a valid numeric format.

 * @example
 * // Example 1: Input with a decimal part
 * const result1 = removeDecimalPart(new BigNumber("123.45"));
 * console.log(result1.toString()); // Outputs: "123"
 * // Outputs: Warning - Decimal part of input (123.45) has been stripped.
 *
 * @example
 * // Example 2: Input without a decimal part
 * const result2 = removeDecimalPart(new BigNumber("678"));
 * console.log(result2.toString()); // Outputs: "678"
 * // Outputs: No warning since there is no decimal part.
 */
export function removeDecimalPart(input: BigNumber): BigNumber {
  if (isNaN(input.toNumber())) {
    throw new Error("Invalid numeric format: Resulting BigNumber is NaN.");
  }

  const inputStr = input.toString();
  const decimalIndex = inputStr.indexOf(".");

  if (decimalIndex !== -1) {
    const strippedDecimal = inputStr.substring(0, decimalIndex);
    const decimalPart = inputStr.substring(decimalIndex + 1);
    console.warn(`Decimal part (${decimalPart}) of input (${inputStr}) has been stripped.`);

    return new BigNumber(strippedDecimal);
  }

  return input;
}
