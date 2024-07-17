import { Auth } from "../auth/Auth";
import { BE_URL } from "../config/config";
import {
  jsonFetch,
  signedJsonFetch,
  unsignedMultipartRequest,
} from "../util/fetch";
import {
  TokenStatus,
  createCoinRequestBodySchema,
  CreateTokenRequestBody,
  QueryTokensRequestParams,
} from "./schemas/token-schemas";
import {
  CreateTokenResponse,
  GetTokenResponse,
  QueryTokensResponse,
  UploadFileResponse,
} from "./types";

/**
 * Service class for handling coin-related operations.
 */
export class TokenAPI {
  /**
   * Constructs a new CoinService instance.
   * @param {string} url - The base URL for the backend service, defaults to BE_URL.
   */
  constructor(private url = BE_URL) {}

  /**
   * Fetches data about a specific coin.
   * @param {TokenStatus} status - The status of the coin that you want to fetch (LIVE or PRESALE)
   * @param {string} tokenAddress - The type of coin to query.
   * @throws Will throw an error if authentication session is not active.
   * @return {Promise<GetTokenResponse>} A promise that resolves with the coin data.
   */
  getToken(
    status: TokenStatus,
    tokenAddress: string
  ): Promise<GetTokenResponse> {
    return jsonFetch(
      `${
        this.url
      }/${status.toLowerCase()}/sol/token?tokenAddress=${tokenAddress}`,
      {
        method: "GET",
      }
    );
  }

  /**
   * Queries coins based on specified parameters.
   * @param {QueryCoinsRequestParams} params - The query parameters to filter coins.
   * @throws Will throw an error if authentication session is not active.
   * @return {Promise<QueryTokensResponse>} A promise that resolves with the queried coin data.
   */
  queryTokens(params: QueryTokensRequestParams): Promise<QueryTokensResponse> {
    const queryParams = new URLSearchParams(params as Record<string, string>);
    return jsonFetch(
      `${
        this.url
      }/${params.status.toLowerCase()}/sol/tokens?${queryParams.toString()}`,
      {
        method: "GET",
      }
    );
  }

  // /**
  //  * Create coin
  //  * @param {CreateTokenRequestBody} params - The create coin request payload.
  //  * @throws Will throw an error if authentication session is not active.
  //  * @return {Promise<CreateTokenResponse>} A promise that resolves with the queried coin data.
  //  */
  // createToken(params: CreateTokenRequestBody): Promise<CreateTokenResponse> {
  //   if (!Auth.currentSession)
  //     throw new Error(
  //       "You don't have any active session, please run the Auth.refreshSession"
  //     );
  //   return signedJsonFetch(`${this.url}/sol/token`, Auth.currentSession, {
  //     method: "POST",
  //     body: createCoinRequestBodySchema.parse(params),
  //   });
  // }

  /**
   * uploadFile to ipfs
   * @param {File} file The URL to which the request is sent.
   * @return {Promise<UploadFileResponse>} A promise that resolves with the response of the fetch request.
   */
  // uploadFile(file: File): Promise<UploadFileResponse> {
  //   return unsignedMultipartRequest(`${this.url}/upload-image`, file);
  // }
}
