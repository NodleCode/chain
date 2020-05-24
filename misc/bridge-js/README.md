# Bridge
A NodeJS service to bridge the Stellar Nodle Cash to the chain itself.

## Design
This is a **one way** bridge that require users to destroy their coins to have them recreated on the new chain. A another two way bridge might be developed in the future.

## Deployment
1. `npx firebase functions:config:set nodle.chainseed="SUBSTRATE_SEED" nodle.nodeendpoint="wss://arcadia1.nodleprotocol.io" stellar.horizonurl="https://horizon-testnet.stellar.org" stellar.issuer="STELLAR_ADDR" stellar.code="STELLAR_ASSET_CODE" stellar.dest="STELLAR_ADDR"`.
2. `npx firebase deploy`.

## API Usage
1. Register memo: `curl -X POST -H "Content-Type:application/json" https://us-central1-nodle-chain-d08ab.cloudfunctions.net/registerMemo -d '{"nodlePublicKey":"5CB5B5dW14sF3cNakCZtA5gGMdxKzaopgsBBrrU5qYT5xj3F"}` which produces a reply of the type `{"memoHash":"E85DF90EFC4E230A63360E229A20394EEB32ED91CFFC6C516514304D1A28A776","destination":"GDACXUPELDAZ7HS7QYN4EPIXM7BAINJ4VUACBMVB3KZR3NNUGZJBBVQ3"}`. The `memoHash` is an **hexadecimal encoded** hash to be added as a field of the type `MEMO_HASH` inside the stellar transaction.
2. Send a payment transaction to `destination` as given by the previous reply along with the memo (`hash` type) given by `memoHash`. Save the transaction hash as you will need it later.
3. Prove transaction: `curl -X POST -H "Content-Type:application/json" https://us-central1-nodle-chain-d08ab.cloudfunctions.net/proveTransaction -d '{"txHash":"f60d111607c39509a1420000d27941bf9d327782b5fbdbb4735683e91780b2c2"}'`.
4. Wait a moment, for this specific bridge we execute all transfers by batches every **1 minutes**.