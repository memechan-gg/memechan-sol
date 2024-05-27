import { BE_URL } from "../config/config";
import { jsonFetch } from "../util/fetch";

export interface AuthParams {
  url: string;
}

export interface IAMCredentials {
  accessKeyId: string;
  secretAccessKey: string;
  sessionToken?: string;
  expiration?: Date;
}


/**
 * Class to handle authentication processes with SUI wallet.
 */
export class Auth {
  /**
   * Current session credentials or undefined if no session is active.
   * @type {IAMCredentials | undefined}
   */
  static currentSession: IAMCredentials | undefined;

  /**
   * Create an authentication service instance.
   * @param {string} url - The base URL for the backend service.
   */
  constructor(private url = BE_URL) {}

  /**
   * Requests a message to sign for initiating an authentication session.
   * @param {string} walletAddress - The wallet address to authenticate.
   * @return {Promise<string>} A promise that resolves to the message to be signed.
   */
  async requestMessageToSign(walletAddress: string): Promise<string> {
    const { textToSign } = await jsonFetch(`${this.url}/auth/request-token`, {
      method: "POST",
      body: {
        walletAddress,
      },
    });
    return textToSign;
  }

  /**
   * Refreshes or creates a new authentication session based on a signed message.
   * The returned credentials has an expiration of 1h.
   * To prevent the user from having to re sign the message multiple times,
   * you can use the signedMessage as a refresh token.
   * The refresh token has an expiration of 1 year.
   * @param {Object} sessionData - The wallet address and signed message.
   * @param {string} sessionData.walletAddress - The wallet address of the user.
   * @param {string} sessionData.signedMessage - The message signed by the user's private key.
   * @return {Promise<IAMCredentials>} A promise that resolves to the session credentials.
   */
  async refreshSession({
    walletAddress,
    signedMessage,
  }: {
    walletAddress: string;
    signedMessage: string;
  }): Promise<IAMCredentials> {
    const { AccessKeyId, SecretAccessKey, SessionToken } = await jsonFetch(`${this.url}/auth/issue-credentials`, {
      method: "POST",
      body: {
        walletAddress,
        messageSignature: signedMessage,
      },
    });
    Auth.currentSession = {
      accessKeyId: AccessKeyId,
      secretAccessKey: SecretAccessKey,
      sessionToken: SessionToken,
    };
    return Auth.currentSession;
  }
}
