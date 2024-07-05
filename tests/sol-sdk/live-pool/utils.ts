/**
 * Returns the numerator and denominator of a given number.
 *
 * This function converts a number into its fractional form by
 * determining the appropriate numerator and denominator.
 *
 * @param {number} num - The input number to be converted.
 * @returns {{ numerator: number, denominator: number }} An object containing the numerator and denominator.
 *
 * @example
 * // Returns { numerator: 5, denominator: 1000 }
 * getNumeratorAndDenominator(0.005);
 *
 * @example
 * // Returns { numerator: 2, denominator: 10 }
 * getNumeratorAndDenominator(0.2);
 *
 * @example
 * // Returns { numerator: 14, denominator: 10 }
 * getNumeratorAndDenominator(1.4);
 *
 * @example
 * // Returns { numerator: 28, denominator: 1 }
 * getNumeratorAndDenominator(28);
 *
 * @example
 * // Returns { numerator: 503231, denominator: 10000 }
 * getNumeratorAndDenominator(50.3231);
 */
export function getNumeratorAndDenominator(num: number): { numerator: number; denominator: number } {
  // Convert the number to a string
  const numStr = num.toString();

  // Check if the number has a fractional part
  if (numStr.includes(".")) {
    // Split the string into integer and fractional parts
    const [integerPart, fractionalPart] = numStr.split(".");

    // The length of the fractional part determines the multiplier for the denominator
    const denominator = Math.pow(10, fractionalPart.length);

    // The numerator is the integer part combined with the fractional part
    const numerator = parseInt(integerPart + fractionalPart, 10);

    return { numerator, denominator };
  } else {
    // If there's no fractional part, the denominator is 1
    return { numerator: num, denominator: 1 };
  }
}
