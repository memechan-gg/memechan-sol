import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MemechanSol } from "../target/types/memechan_sol";
import * as newPool from "./endpoints/new"
import * as swapY from "./endpoints/swap_y"

describe("memechan-sol", () => {
  newPool.test();
  swapY.test();
});