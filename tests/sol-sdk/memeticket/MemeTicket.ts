import { IdlAccounts, Program } from "@coral-xyz/anchor";
import {
  GetProgramAccountsFilter,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import BigNumber from "bignumber.js";
import { MemechanClient } from "../MemechanClient";

import {
  BoundMerge,
  CloseArgs,
  GetBoundMergeTransactionArgs,
  GetCloseTransactionArgs,
  GetStakingMergeTransactionArgs,
  ParsedMemeTicket,
  StakingMerge,
} from "./types";
import { getOptimizedTransactions } from "./utils";
import { IdlAccount } from "@coral-xyz/anchor/dist/cjs/idl";
import { MemechanSol } from "../../../target/types/memechan_sol";
import { memechan } from "../../helpers";

export type MemeTicketFields = IdlAccounts<MemechanSol>["memeTicket"];

export class MemeTicket {
  public constructor(public id: PublicKey, public client: MemechanClient) {
    //
  }

  public async fetch(program = this.client.memechanProgram) {
    return program.account.memeTicket.fetch(this.id);
  }

  public static async all(program: Program<MemechanSol>) {
    return program.account.memeTicket.all();
  }

  public async getBoundMergeTransaction({
    transaction,
    pool,
    ticketsToMerge,
    user,
  }: GetBoundMergeTransactionArgs): Promise<Transaction> {
    const tx = transaction ?? new Transaction();

    for (const ticket of ticketsToMerge) {
      const mergeInstruction = await this.client.memechanProgram.methods
        .boundMergeTickets()
        .accounts({
          owner: user.publicKey,
          pool: pool,
          ticketFrom: ticket.id,
          ticketInto: this.id,
        })
        .instruction();

      tx.add(mergeInstruction);
    }

    return tx;
  }

  public async boundMerge(input: BoundMerge): Promise<MemeTicket> {
    const mergeTransaction = await this.getBoundMergeTransaction(input);

    const optimizedTransactions = getOptimizedTransactions(
      mergeTransaction.instructions,
      input.user.publicKey
    );
    console.log("optimizedTransactions length:", optimizedTransactions.length);

    for (const tx of optimizedTransactions) {
      const signature = await sendAndConfirmTransaction(
        this.client.connection,
        tx,
        [input.user],
        {
          commitment: "confirmed",
          skipPreflight: true,
        }
      );
      console.log("bound merge signature:", signature);
    }

    return this;
  }

  public async getStakingMergeTransaction({
    staking,
    ticketsToMerge,
    user,
    transaction,
  }: GetStakingMergeTransactionArgs): Promise<Transaction> {
    const tx = transaction ?? new Transaction();

    for (const ticket of ticketsToMerge) {
      const mergeInstruction = await this.client.memechanProgram.methods
        .stakingMergeTickets()
        .accounts({
          owner: user.publicKey,
          staking: staking,
          ticketFrom: ticket.id,
          ticketInto: this.id,
        })
        .instruction();

      tx.add(mergeInstruction);
    }

    return tx;
  }

  public async stakingMerge(input: StakingMerge): Promise<MemeTicket> {
    const mergeTransaction = await this.getStakingMergeTransaction(input);

    const optimizedTransactions = getOptimizedTransactions(
      mergeTransaction.instructions,
      input.user.publicKey
    );

    for (const tx of optimizedTransactions) {
      const signature = await sendAndConfirmTransaction(
        this.client.connection,
        tx,
        [input.user],
        {
          commitment: "confirmed",
          skipPreflight: true,
        }
      );
      console.log("staking merge signature:", signature);
    }

    return this;
  }

  public async getCloseTransaction({
    user,
    transaction,
  }: GetCloseTransactionArgs): Promise<Transaction> {
    const tx = transaction ?? new Transaction();

    const closeInstruction = await this.client.memechanProgram.methods
      .closeTicket()
      .accounts({
        owner: user.publicKey,
        ticket: this.id,
      })
      .instruction();

    tx.add(closeInstruction);

    return tx;
  }

  public async close(input: CloseArgs): Promise<MemeTicket> {
    const closeTransaction = await this.getCloseTransaction(input);

    const signature = await sendAndConfirmTransaction(
      this.client.connection,
      closeTransaction,
      [input.user],
      {
        commitment: "confirmed",
        skipPreflight: true,
      }
    );
    console.log("close meme ticket signature:", signature);

    return this;
  }

  /**
   * Fetches all tickets for provided pool id
   */
  public static async fetchRelatedTickets(
    pool: PublicKey,
    client: MemechanClient
  ): Promise<MemeTicketFields[]> {
    const program = client.memechanProgram;
    const filters: GetProgramAccountsFilter[] = [
      {
        memcmp: {
          bytes: pool.toBase58(),
          offset: 40,
        },
      },
    ];

    const fetchedTickets = await program.account.memeTicket.all(filters);
    const tickets = fetchedTickets.map((ticket) => ticket.account);
    return tickets;
  }

  public static async fetchTicketsByUser(
    pool: PublicKey,
    client: MemechanClient,
    user: PublicKey
  ): Promise<ParsedMemeTicket[]> {
    const program = client.memechanProgram;
    const filters: GetProgramAccountsFilter[] = [
      {
        memcmp: {
          bytes: pool.toBase58(),
          offset: 40,
        },
      },
      {
        memcmp: {
          bytes: user.toBase58(),
          offset: 8,
        },
      },
    ];

    const fetchedTickets = await program.account.memeTicket.all(filters);

    const parsedTickets = fetchedTickets.map((ticket) => {
      const jsonTicket = ticket.toString();

      return {
        id: ticket.publicKey,
        jsonFields: jsonTicket,
        fields: ticket.account,
      };
    });

    return parsedTickets;
  }

  public static async fetchAvailableTicketsByUser(
    pool: PublicKey,
    client: MemechanClient,
    user: PublicKey
  ) {
    const tickets = await MemeTicket.fetchTicketsByUser(pool, client, user);
    const currentTimestamp = Date.now();

    const availableTickets = tickets.filter((ticket) => {
      const unlockTicketTimestamp = +ticket.jsonFields.untilTimestamp;

      return currentTimestamp >= unlockTicketTimestamp;
    });

    const availableAmount = availableTickets
      .reduce((amount: BigNumber, ticket) => {
        amount = amount.plus(ticket.jsonFields.amount);
        return amount;
      }, new BigNumber(0))
      .toString();

    return { tickets: availableTickets, availableAmount };
  }
}
