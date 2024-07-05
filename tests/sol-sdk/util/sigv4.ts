import { Sha256 } from "@aws-crypto/sha256-browser";
import type { AwsCredentialIdentity, Provider } from "@aws-sdk/types";
import { HttpRequest } from "@smithy/protocol-http";
import { SignatureV4 } from "@smithy/signature-v4";
import { getFetchFn } from "./get-fetch";
import { getHeaders } from "./get-headers";

export type SignedFetcherOptions = {
  service: string;
  region?: string;
  credentials: AwsCredentialIdentity | Provider<AwsCredentialIdentity>;
  fetch?: typeof fetch;
};

export type CreateSignedFetcher = (init: SignedFetcherOptions) => typeof fetch;

export const createSignedFetcher: CreateSignedFetcher = (opts: SignedFetcherOptions): typeof fetch => {
  const fetchFn = getFetchFn(opts.fetch);

  return async (input, init?) => {
    const service = opts.service;
    const region = opts.region || "us-east-1";
    const credentials = opts.credentials;

    const url = new URL(typeof input === "string" ? input : input instanceof URL ? input.href : input.url);

    const headers = getHeaders(init?.headers);
    headers.set("host", url.host);

    const request = new HttpRequest({
      hostname: url.hostname,
      path: url.pathname,
      protocol: url.protocol,
      port: url.port ? Number(url.port) : undefined,
      username: url.username,
      password: url.password,
      method: init?.method?.toUpperCase() ?? "GET", // method must be uppercase
      body: init?.body,
      query: Object.fromEntries(url.searchParams.entries()),
      fragment: url.hash,
      headers: Object.fromEntries(headers.entries()),
    });

    const signer = new SignatureV4({
      credentials,
      service,
      region,
      sha256: Sha256,
    });

    const signedRequest = await signer.sign(request);

    return fetchFn(url, {
      ...init,
      headers: signedRequest.headers,
      body: signedRequest.body,
      method: signedRequest.method,
    });
  };
};
