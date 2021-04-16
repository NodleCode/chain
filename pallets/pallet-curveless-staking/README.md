# Curveless Staking
A fork of Substrate Frame v2 `pallet-staking`. The main differences are as follows:
- no inflationary reward curve
- the pallet has a dedicated account id to receive potential fungible tokens
- the pallet tracks rewards deposited in its account on a per era basis
- rewards from a given era are distributed between the validators and their nominators while respecting the traditional parameters

This is especially useful if you want to create non-inflationary reward models, for instance by redistributing transaction fees and other forms of rewards.

Example integrations:
- [Ternoa](https://github.com/capsule-corp-ternoa/chain/pull/22)