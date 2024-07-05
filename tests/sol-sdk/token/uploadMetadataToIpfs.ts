import { TokenAPI } from "./TokenAPI";
import { TokenMetadata } from "./types";

export async function uploadMetadataToIpfs(metadata: TokenMetadata): Promise<string> {

const metadataBlob = new Blob([JSON.stringify(metadata)], { type: "application/json" });
  const fileName = "metadata.json";
  const metadataFile = new File([metadataBlob], fileName);

  const tokenApi = new TokenAPI();
  const fileUploadResult = await tokenApi.uploadFile(metadataFile);

  const metadataUri = "https://cf-ipfs.com/ipfs/" + fileUploadResult.IpfsHash;

  console.log("metadataUri: " + metadataUri);

  return metadataUri;
}