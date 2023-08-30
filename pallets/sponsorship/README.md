# Sponsorship Pallet

This pallet provides the logic needed for creating and updating sponsorship pots which allow sponsors to provide funds 
for a subset of their registered users transactions.
This pallet complements the logic in the transaction payment pallet and must be used in conjunction with it.

This would allow sponsored accounts to transact without paying fees. The sponsors could also provide the fund that would 
be needed to be reserved for some of the transactions. For example, creating an NFT collection requires reserving a 
deposit which the sponsored account might not be able to afford. The sponsor can provide that too while maintaining the 
right to get that fund back when the sponsored account destroys the collection. 

One important aspect of this sponsorship is that the actions that the sponsored accounts do on chain through this pallet 
will still remain originated from their own wallets.

Another aspect of this kind of sponsorship is its full control over who can use this feature and up to what level and 
for which actions.

The action categories however are coming from pre-defined categories that are configurable in the runtime. 
Sponsor can only choose from those categories but cannot define a new one.