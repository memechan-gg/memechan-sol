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

export interface MemechanClientConfigArgs {}

export class MemechanClient {
  public wallet: Wallet;
  public connection: Connection;
  public memechanProgram: Program<MemechanSol>;
  public anchorProvider: AnchorProvider;
  public heliusApiUrl: string;

  constructor(private config: MemechanClientConfigArgs) {
    this.connection = provider.connection;

    this.anchorProvider = provider;

    console.log("program id: " + MEMECHAN_PROGRAM_ID);
    console.log("connection rpc: " + this.connection.rpcEndpoint);

    this.memechanProgram = memechan;
  }
}
