import { BE_URL } from "../config/config";
import { jsonFetch } from "../util/fetch";
import { SolanaLivePool, SolanaSeedPool } from "./schemas/pools-schema";
import { QueryAllLivePoolsResponse, QueryAllSeedPoolsResponse, QueryAllStakingPoolsResponse } from "./types";

/**
 * Service class for handling pool-related operations.
 */
export class PoolAPI {
  /**
   * Constructs a new PoolAPI instance.
   * @param {string} url - The base URL for the backend service, defaults to BE_URL.
   */
  constructor(private url = BE_URL) {}

  /**
   * Retrieves all seed pools with optional pagination.
   * @param {string} [paginationToken] - A token for pagination, if more results are available beyond the first page.
   * @return {Promise<QuerySolanaSeedPoolsParams>} A Promise that resolves with the list of all seed pools.
   */
  getAllSeedPools(paginationToken?: string): Promise<QueryAllSeedPoolsResponse> {
    return jsonFetch(`${this.url}/sol/seed-pools${paginationToken ? "?paginationToken=" + paginationToken : ""}`, {
      method: "GET",
    });
  }

  /**
   * Retrieves seed pool filtered by coin type.
   * @param {string} tokenAddress - The type of coin for filtering seed pools.
   * @return {Promise<SolanaSeedPool>} A Promise that resolves with the list of seed pools matching the specified coin type.
   */
  getSeedPoolByTokenAddress(tokenAddress: string): Promise<SolanaSeedPool> {
    return jsonFetch(`${this.url}/sol/seed-pools?tokenAddress=${tokenAddress}`, {
      method: "GET",
    });
  }

  /**
   * Retrieves live pool by coin type.
   * @param {string} tokenAddress - The type of coin for filtering seed pools.
   * @return {Promise<SolanaLivePool>} A Promise that resolves with the live pool matching the specified coin type.
   */
  getLivePoolByTokenAddress(tokenAddress: string): Promise<SolanaLivePool> {
    return jsonFetch(`${this.url}/sol/live-pools?coinType=${tokenAddress}`, {
      method: "GET",
    });
  }

  /**
   * Retrieves staking pool by coin type.
   * @param {string} tokenAddress - The type of coin for filtering seed pools.
   * @return {Promise<QueryAllStakingPoolsResponse>} A Promise that resolves with the live pool matching the specified coin type.
   */
  getStakingPoolByCoinType(tokenAddress: string): Promise<QueryAllStakingPoolsResponse> {
    return jsonFetch(`${this.url}/sol/staking-pools?tokenAddress=${tokenAddress}`, {
      method: "GET",
    });
  }

  /**
   * Retrieves seed pools filtered by coin type.
   * @param {string} [paginationToken] - A token for pagination, if more results are available beyond the first page.
   * @return {Promise<QueryAllLivePoolsResponse>} A Promise that resolves with the list of seed pools matching the specified coin type.
   */
  getLivePools(paginationToken?: string): Promise<QueryAllLivePoolsResponse> {
    return jsonFetch(`${this.url}/sol/live-pools${paginationToken ? "?paginationToken=" + paginationToken : ""}`, {
      method: "GET",
    });
  }

  /**
   * Retrieves seed pools filtered by coin type.
   * @param {string} [paginationToken] - A token for pagination, if more results are available beyond the first page.
   * @return {Promise<QueryAllStakingPoolsResponse>} A Promise that resolves with the list of seed pools matching the specified coin type.
   */
  getStakingPools(paginationToken?: string): Promise<QueryAllStakingPoolsResponse> {
    return jsonFetch(`${this.url}/sol/staking-pools${paginationToken ? "?paginationToken=" + paginationToken : ""}`, {
      method: "GET",
    });
  }
}
