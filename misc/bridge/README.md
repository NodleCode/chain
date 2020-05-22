# Bridge
A NodeJS service to bridge the Stellar Nodle Cash to the chain itself.

## Design
This is a **one way** bridge that require users to destroy their coins to have them recreated on the new chain. A another two way bridge might be developed in the future.

## Deployment
- config vars: gateway, seed
- firebase deploy

## API Usage
1. Register memo
2. Send transaction
3. Prove transaction
4. Wait every x minutes