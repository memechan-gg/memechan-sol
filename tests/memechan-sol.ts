import * as newPool from "./endpoints/new";
import * as swapY from "./endpoints/swap_y";
import * as swapX from "./endpoints/swap_x";
import * as goLive from "./endpoints/go_live";
import * as fees from "./endpoints/fees";
import * as tickets from "./endpoints/tickets";
import * as staking from "./endpoints/staking";

import { airdrop, payer, provider } from "./helpers";

describe("memechan-sol", () => {
  provider.opts.skipPreflight = true;
  newPool.test();
  swapY.test();
  swapX.test();
  goLive.test();
  fees.test();
  tickets.test();
  staking.test();
});
