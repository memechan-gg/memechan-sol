import { sleepTime } from ".";

/**
 * Parameters for the retry function.
 */
interface RetryParams<T> {
  fn: () => Promise<T>;
  retries?: number;
  delay?: number;
  functionName: string;
}

/**
 * Retries a given asynchronous function a specified number of times with a delay between attempts.
 *
 * @param {RetryParams<T>} params - The parameters for the retry function.
 * @returns {Promise<T>} - The result of the asynchronous function if it succeeds.
 * @throws {Error} - Throws the error if all retry attempts fail.
 */
export async function retry<T>({ fn, retries = 3, delay = 1000, functionName }: RetryParams<T>): Promise<T> {
  for (let i = 0; i < retries; i++) {
    try {
      return await fn();
    } catch (error) {
      console.warn(`Retry attempt ${i + 1} for function ${functionName}`);
      if (i === retries - 1) {
        console.warn(`All retries failed for function ${functionName}`);
        throw error;
      }
      sleepTime(delay);
    }
  }
  // This line is unreachable, but TypeScript needs assurance that the function returns or throws.
  throw new Error(`Unreachable code in retry function for ${functionName}`);
}
