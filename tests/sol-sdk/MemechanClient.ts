import {
  AnchorProvider,
  Program,
  Wallet,
  setProvider,
} from "@coral-xyz/anchor";
import { Connection, ConnectionConfig, PublicKey } from "@solana/web3.js";
import { MemechanSol } from "../../target/types/memechan_sol";
import { MEMECHAN_PROGRAM_ID, memechan } from "./config/config";
import { provider } from "../helpers";

export interface MemechanClientConfigArgs {
  wallet: Wallet;
  rpcConnectionConfig?: ConnectionConfig;
  rpcApiUrl: string;
  wssApiUrl: string;
  heliusApiUrl: string;
  isTest: boolean;
}

export class MemechanClient {
  public wallet: Wallet;
  public connection: Connection;
  public memechanProgram: Program<MemechanSol>;
  public anchorProvider: AnchorProvider;
  public heliusApiUrl: string;

  constructor(private config: MemechanClientConfigArgs) {
    const {
      wallet,
      isTest,
      rpcApiUrl,
      rpcConnectionConfig,
      wssApiUrl,
      heliusApiUrl,
    } = config;

    this.wallet = wallet;
    // this.connection = new Connection(rpcApiUrl, {
    //   httpAgent: isTest ? false : undefined,
    //   commitment: "confirmed",
    //   //wsEndpoint: wssApiUrl,
    //   confirmTransactionInitialTimeout: 1000,
    //   ...(rpcConnectionConfig ? rpcConnectionConfig : {}),
    // });
    this.connection = provider.connection;

    this.heliusApiUrl = heliusApiUrl;
    // const provider = new AnchorProvider(this.connection, wallet, {
    //   commitment: "confirmed",
    // });
    this.anchorProvider = provider;

    console.log("program id: " + MEMECHAN_PROGRAM_ID);
    console.log("connection rpc: " + this.connection.rpcEndpoint);

    this.memechanProgram = memechan;
  }
}
