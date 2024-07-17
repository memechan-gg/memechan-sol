import { TokenAPI } from "./TokenAPI";
import { TokenMetadata } from "./types";

export async function uploadMetadataToIpfs(
  metadata: TokenMetadata
): Promise<string> {
  const metadataUri = "https://cf-ipfs.com/ipfs/" + "11111";

  // console.log("metadataUri: " + metadataUri);

  return metadataUri;
}
