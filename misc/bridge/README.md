# Bridge
A NodeJS service to bridge the Stellar Nodle Cash to the chain itself.

## Design
This is a **one way** bridge that require users to destroy their coins to have them recreated on the new chain. A another two way bridge might be developed in the future.

## Deployment
1. `npx firebase functions:config:set nodle.coinsdest="STELLAR_ADDR" nodle.chainseed="SUBSTRATE_SEED"`
2. `npx firebase deploy`

## API Usage
1. Register memo: `curl -X POST -H "Content-Type:application/json" https://us-central1-nodle-chain-d08ab.cloudfunctions.net/registerMemo -d '{"nodlePublicKey":"5CB5B5dW14sF3cNakCZtA5gGMdxKzaopgsBBrrU5qYT5xj3F"}` which produces a reply of the type `{"memoHash":"3F7716891E6B9D128A111F05B25C984A","destination":"GAVVGIZPXO4LRQ7DZT2MWBYR5TC4ZXCQUNRJOOULSQC6SN3Y73NI3I26"}`. The `memoHash` is an **hexadecimal encoded** hash to be added as a field of the type `MEMO_HASH` inside the stellar transaction.
2. Send a payment transaction to `destination` as given by the previous reply along with the memo (`hash` type) given by `memoHash`. Save the transaction hash as you will need it later.
3. Prove transaction
4. Wait every x minutes