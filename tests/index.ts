import chai from "chai";

import * as createVestingSchedule from "./endpoints/create-vesting-schedule";
import * as changeVestingWallet from "./endpoints/change-vestee-wallet";
import * as updateVestedTokens from "./endpoints/update-vested-tokens";

import { airdrop, provider } from "./helpers";

describe("vesting-treasury", () => {
    createVestingSchedule.test();
    changeVestingWallet.test();
    updateVestedTokens.test();


  before("airdrop SOL to provider wallet", async () => {
    await airdrop(provider.wallet.publicKey);
  });
});
