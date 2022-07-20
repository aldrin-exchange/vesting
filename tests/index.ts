import chai from "chai";

import * as createVestingSchedule from "./endpoints/create-vesting-schedule";

import { airdrop, provider } from "./helpers";

describe("vesting-treasury", () => {
    createVestingSchedule.test();


  before("airdrop SOL to provider wallet", async () => {
    await airdrop(provider.wallet.publicKey);
  });
});
