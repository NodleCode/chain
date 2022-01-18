**Introduction**

Use wNodl-pallet to initiate wrapping an individual's fund into wNodl (Contract address: 
`0x5f6d994e6ba25a0a23efae15e46a6e79527bdf3f`) on Ethereum.
This pallet would not interact with Ethereum directly. Rather it would create on-chain events which would be picked on 
by another collaborating off-chain oracle called wNodl-bot. 
wNodle-bot will be responsible to make the actual transaction on Ethereum and when successfully done, confirm it 
on-chain by settling the funds back on this pallet. wNodl-bot will provide a proof which will be the corresponding 
Ethereum transaction hash. That hash will be published as an event on chain.