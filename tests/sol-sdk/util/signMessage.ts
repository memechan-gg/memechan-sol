import { Keypair } from "@solana/web3.js";
import nacl from "tweetnacl";
import { decodeUTF8, encodeBase64 } from "tweetnacl-util";

/**
 * Helper method to sign a message using Solana's web3.js keypair.
 * @param {string} message - The message to be signed.
 * @param {Keypair} keypair - The Solana keypair used for signing.
 * @returns {string} - The signed message in Base64 format.
 */
export async function signMessage(message: string, keypair: Keypair) {
  // Step 1: Encode the message
  const messageBytes = decodeUTF8(message);

  // Step 2: Sign the message
  const signature = nacl.sign.detached(messageBytes, keypair.secretKey);

  // Step 3: Verify the signature (optional)
  const isValid = nacl.sign.detached.verify(
    messageBytes,
    signature,
    keypair.publicKey.toBytes()
  );

  if (!isValid) {
    throw new Error("Signature verification failed");
  }

  // Return the signed message in Base64 format
  return encodeBase64(signature);
}