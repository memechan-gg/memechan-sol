import { expect } from "chai";
import { BoundPool } from "../BoundPool";

export function test() {
  describe("create_bound_pool", () => {
    it("creates bound pool", async () => {
      const boundPool = await BoundPool.new();
      const info = await boundPool.fetch();
      console.log(info);
    });
  });
}
