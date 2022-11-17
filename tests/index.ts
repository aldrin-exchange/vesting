import chai from "chai";

import * as createVestingSchedule from "./endpoints/create-vesting-schedule";
import * as changeVestingWallet from "./endpoints/change-vestee-wallet";
import * as updateVestedTokens from "./endpoints/update-vested-tokens";
import * as fundVestingVault from "./endpoints/fund-vesting-vault";
import * as withdrawVestedTokens from "./endpoints/withdraw-vested-tokens";
import * as closeVestingSchedule from "./endpoints/close-vesting-schedule";

import { airdrop, provider } from "./helpers";

describe("vesting-treasury", () => {
    createVestingSchedule.test();
    changeVestingWallet.test();
    updateVestedTokens.test();
    fundVestingVault.test();
    withdrawVestedTokens.test();
    closeVestingSchedule.test();


  before("airdrop SOL to provider wallet", async () => {
    await airdrop(provider.wallet.publicKey);
  });
});
