import { expect } from "chai";
import { BoundPool } from "../bound_pool";

export function test() {
  describe.skip("create_bound_pool", () => {
    it.skip("creates bound pool", async () => {
      const boundPool = await BoundPool.new();
      const info = await boundPool.fetch();
      console.log(info)
    });
  });
}