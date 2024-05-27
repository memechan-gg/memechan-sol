import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MemechanSol } from "../target/types/memechan_sol";
import * as newPool from "./endpoints/new";
import * as swapY from "./endpoints/swapY";
import * as swapX from "./endpoints/swapX";
import * as goLive from "./endpoints/init";
import * as initStakingPool from "./endpoints/initStakingPool";
import * as fees from "./endpoints/fees";
import * as tickets from "./endpoints/tickets";
import * as staking from "./endpoints/staking";

import { provider } from "./helpers";

describe("memechan-sol", () => {
  provider.opts.skipPreflight = true;
  // newPool.test();
  // swapY.test();
  // swapX.test();
  // initStakingPool.test();
  goLive.test();
  // fees.test();
  // tickets.test();
  // staking.test();
});
