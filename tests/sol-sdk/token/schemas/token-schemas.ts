import { z, ZodRawShape } from "zod";

export const paginatedResultSchema = <T extends ZodRawShape>(
  result: z.ZodObject<T>
) =>
  z.object({
    paginationToken: z.string().nullish(),
    result: z.array(result),
  });

export const solanaTokensSortableColumns = z
  .literal("marketcap")
  .or(z.literal("creationTime"))
  .or(z.literal("lastReply"));

export const tokenStatus = z.literal("LIVE").or(z.literal("PRESALE"));

export const solanaSocialLinks = z.object({
  telegram: z.string().nullish(),
  website: z.string().nullish(),
  twitter: z.string().nullish(),
  discord: z.string().nullish(),
});

export const createCoinRequestBodySchema = z.object({
  txDigest: z.string(),
  socialLinks: solanaSocialLinks.nullish(),
});

export const querySolanaTokensRequestParamsSchema = z.object({
  sortBy: solanaTokensSortableColumns,
  status: z.literal("PRESALE").or(z.literal("LIVE")),
  direction: z.literal("asc").or(z.literal("desc")),
  paginationToken: z.string().nullish(),
});

export const solanaTokenSchema = z.object({
  name: z.string(),
  address: z.string(),
  decimals: z.number(),
  symbol: z.string(),
  description: z.string(),
  image: z.string(),
  lastReply: z.number(),
  marketcap: z.number(),
  creator: z.string(),
  status: tokenStatus,
  socialLinks: solanaSocialLinks.nullish(),
  txDigest: z.string(),
  creationTime: z.number(),
});

export const solanaTokenMetadata = z.object({
  decimals: z.number(),
  description: z.string(),
  icon_url: z.string(),
  name: z.string(),
  symbol: z.string(),
});

export const socialLinks = z.object({
  twitter: z.string().nullish(),
  discord: z.string().nullish(),
  telegram: z.string().nullish(),
  website: z.string().nullish(),
});

export const createTokenRequestBodySchema = z.object({
  txDigest: z.string(),
  socialLinks: socialLinks.nullish(),
});

export type SolanaTokenMetadata = z.infer<typeof solanaTokenMetadata>;
export type SolanaToken = z.infer<typeof solanaTokenSchema>;
export type SortableColumn = z.infer<typeof solanaTokensSortableColumns>;
export type QueryTokensRequestParams = z.infer<
  typeof querySolanaTokensRequestParamsSchema
>;
export type CreateTokenRequestBody = z.infer<
  typeof createTokenRequestBodySchema
>;
export type SolanaSocialLinks = z.infer<typeof solanaSocialLinks>;
export type TokenStatus = z.infer<typeof tokenStatus>;
