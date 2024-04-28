import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MemechanSol } from "../target/types/memechan_sol";
import * as newPool from "./endpoints/new"
import * as swapY from "./endpoints/swap_y"
import * as swapX from "./endpoints/swap_x"

describe("memechan-sol", () => {
  newPool.test();
  swapY.test();
  swapX.test();
});