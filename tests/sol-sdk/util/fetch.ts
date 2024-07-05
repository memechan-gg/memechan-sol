import { IAMCredentials } from "../auth/Auth";
import { BE_REGION } from "../config/config";
import { createSignedFetcher } from "./sigv4";


export const jsonFetch = async (
  input: string | URL | globalThis.Request,
  init?: Omit<RequestInit, "body"> & { body?: unknown },
) => {
  let body;
  if (init?.body) {
    body = JSON.stringify(init.body);
  }
  const r = await fetch(input, { ...init, body });
  if (!r.ok) {
    const body = await r.text();
    try {
      throw new Error(JSON.stringify({ body, status: r.statusText }));
    } catch (e) {
      throw new Error(body);
    }
  }
  return r.json();
};

export const unsignedMultipartRequest = async (input: string, file: File) => {
  const formData = new FormData();

  formData.append("file", file);

  try {
    const r = await fetch(input, {
      method: "POST",
      headers: {
        Accept: file.type,
      },
      body: formData,
    });
    if (!r.ok) {
      const body = await r.json();
      throw new Error(JSON.stringify({ body, status: r.statusText }));
    }
    return r.json();
  } catch (error) {
    console.error("Upload failed:", error);
    throw error;
  }
};


export const signedJsonFetch = async (
  input: string,
  credentials: IAMCredentials,
  init?: Omit<RequestInit, "body"> & {
    body?: unknown;
  },
) => {
  const { method, body } = init || {};
  const signedFetch = createSignedFetcher({ service: "execute-api", region: BE_REGION, credentials });
  const r = await signedFetch(input, {
    method,
    body: body ? JSON.stringify(body) : undefined,
    headers: { "Content-Type": "application/json" },
  });
  if (!r.ok) {
    const body = await r.text();
    try {
      throw new Error(JSON.stringify({ body, status: r.statusText }));
    } catch (e) {
      throw new Error(body);
    }
  }
  return r.json();
};

