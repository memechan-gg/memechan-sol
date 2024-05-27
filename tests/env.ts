import "dotenv/config";

if (!process.env.TEST_USER_SECRET_KEY?.length) {
  throw new Error("Empty TEST_USER_SECRET_KEY");
}
if (!process.env.RPC_API_CLUSTER?.length) {
  throw new Error("Empty RPC_API_CLUSTER");
}
if (!process.env.WSS_API_CLUSTER?.length) {
  throw new Error("Empty WSS_API_CLUSTER");
}
if (!process.env.FEE_DESTINATION_ID?.length) {
  throw new Error("Empty FEE_DESTINATION_ID");
}
if (!process.env.USER_ID?.length) {
  throw new Error("Empty USER_ID");
}

export const TEST_USER_SECRET_KEY = process.env.TEST_USER_SECRET_KEY;
export const RPC_API_CLUSTER = process.env.RPC_API_CLUSTER;
export const WSS_API_CLUSTER = process.env.WSS_API_CLUSTER;
export const FEE_DESTINATION_ID = process.env.FEE_DESTINATION_ID;
export const USER_ID = process.env.USER_ID;
export const IS_TEST_ENV = process.env.NODE_ENV === "test";
