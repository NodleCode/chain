# Bridge
A NodeJS service to bridge the Stellar Nodle Cash to the chain itself.

## Design
This is a **one way** bridge that require users to destroy their coins to have them recreated on the new chain. A another two way bridge might be developed in the future.

## Deployment
1. (TODO) config vars: gateway, seed
2. `npx firebase deploy`

## API Usage
1. Register memo: `curl -X POST -H "Content-Type:application/json" https://us-central1-nodle-chain-d08ab.cloudfunctions.net/registerMemo -d '{"nodlePublicKey":"5CB5B5dW14sF3cNakCZtA5gGMdxKzaopgsBBrrU5qYT5xj3F"}` which produces a reply of the type `{"memoHash":"3F7716891E6B9D128A111F05B25C984A"}`. The `memoHash` is an **hexadecimal encoded** hash to be added as a field of the type `MEMO_HASH` inside the stellar transaction.
2. Send transaction
3. Prove transaction
4. Wait every x minutes