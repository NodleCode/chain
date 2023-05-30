import '@polkadot/api-augment';
/* import '@polkadot/api/augment';
import '@polkadot/types/augment';
import '@polkadot/types/lookup'; */
import { describeSuite, beforeAll, expect } from "@moonwall/cli";
import { ALITH_PRIVATE_KEY, BALTATHAR_ADDRESS, CHARLETH_ADDRESS, alith } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { parseEther, ethers, Transaction, Wallet, parseUnits } from "ethers";

describeSuite({
  id: "CMB01",
  title: "Chopsticks test suite",
  foundationMethods: "chopsticks",
  testCases: ({ it, context, log }) => {
    let api: ApiPromise;
    const DUMMY_ACCOUNT = "0x11d88f59425cbc1867883fcf93614bf70e87E854";

    beforeAll(() => {
      api = context.polkadotJs();
    });

    it({
      id: "T01",
      title: "Calling chain constants data",
      test: async () => {
        const specName = api.consts.system.version.specName.toString();
        expect(specName).to.contain("nodle-para");
      },
    });

    it({
      id: "T02",
      title: "Can create new blocks",
      test: async () => {
        const currentHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
        await context.createBlock({ count: 2 });
        const newHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(newHeight - currentHeight).to.be.equal(2);
      },
    });

    it({
      id: "T03",
      title: "Can send balance transfers",
      test: async () => {
        const balanceBefore = (await api.query.system.account(DUMMY_ACCOUNT)).data.free.toBigInt();
        await api.tx.balances.transfer(DUMMY_ACCOUNT, parseEther("1")).signAndSend(alith);
        await context.createBlock();
        const balanceAfter = (await api.query.system.account(DUMMY_ACCOUNT)).data.free.toBigInt();
        expect(balanceBefore < balanceAfter).to.be.true;
      },
    });

    it({
      id: "T5",
      title: "Create block and check events",
      test: async function () {
        const expectEvents = [
          api.events.system.ExtrinsicSuccess,
          api.events.balances.Transfer,
          api.events.system.NewAccount,
          // api.events.authorFilter.EligibleUpdated
        ];

        await api.tx.balances.transfer(CHARLETH_ADDRESS, parseEther("3")).signAndSend(alith);
        await context.createBlock({ expectEvents, logger: log });
      },
    });

    it({
      id: "T6",
      title: "Create block, allow failures and check events",
      test: async function () {
        await api.tx.balances
          .forceTransfer(BALTATHAR_ADDRESS, CHARLETH_ADDRESS, parseEther("3"))
          .signAndSend(alith);
        // await api.tx.balances.transfer(CHARLETH_ADDRESS, parseEther("3")).signAndSend(alith);
        const { result } = await context.createBlock({ allowFailures: true });

        const apiAt = await api.at(result);
        const events = await apiAt.query.system.events();
        expect(
          events.find((evt) => api.events.system.ExtrinsicFailed.is(evt.event)),
          "No Event found in block"
        ).toBeTruthy();
      },
    });
  },
});
