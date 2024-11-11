searchState.loadedDescShard("runtime_eden", 0, "The address format for describing accounts.\nIt’s a 20 byte representation.\nIt’s a 32 byte representation.\nAll pallets included in the runtime as a nested tuple of …\nAll pallets included in the runtime as a nested tuple of …\nAn Aura authority identifier using S/R 25519 as its crypto.\nBlock type as expected by this runtime.\nBlockId type as expected by this runtime.\nComplex storage builder stuff.\nExtrinsic type that has already been checked.\nExecutive: handles dispatch to the various modules.\nIdentify by block header hash.\nBlock header type as expected by this runtime.\nIt’s an account ID (pubkey).\nIt’s an account index.\nIdentify by block number.\nProvides an implementation of <code>PalletInfo</code> to provide …\nUnique identifier of a parachain.\nIt’s some arbitrary raw bytes.\nImplements all runtime apis for the client side.\nThe aggregated runtime call type.\nA reason for placing a freeze on funds.\nA reason for placing a hold on funds.\nAn identifier for each lock placed on funds.\nThe runtime origin type representing the origin of a call.\nA reason for slashing funds.\nAn aggregation of all <code>Task</code> enums across all pallets …\nA Block signed with a Justification\nThe SignedExtension to the basic transaction logic.\nThe payload being signed in transactions.\nUnchecked extrinsic type as expected by this runtime.\nThis runtime version. This should not be thought of as …\nAssimilate the storage for this module into pre-existing …\nFull block.\nBuild the storage out of this builder.\nDecode <code>Self</code> from the given <code>encoded</code> slice and convert <code>Self</code> …\nA chain-specific digest of data useful for light clients …\nThe accompanying extrinsics.\nThe merkle root of the extrinsics.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert to runtime origin with caller being system signed …\nConvert to runtime origin using […\nConvert to runtime origin using […\nConvert to runtime origin, using as filter: …\nConvert to runtime origin using […\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nOptionally convert the <code>DispatchError</code> into the <code>RuntimeError</code>.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nThe function that should be called.\nThe function that should be called.\nGenerate a set of keys with optionally using the given …\nThe block header.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts <code>Self</code> into a <code>Vec</code> of <code>(raw public key, KeyTypeId)</code>.\nBlock justification.\nThe version infromation used to identify this runtime when …\nCreate an <code>Id</code>.\nCreate with system none origin and …\nThe block number.\nThe parent hash.\nCreate with system root origin and …\nThe default version to encode outgoing XCM messages with.\nThe signature, address, number of extrinsics have come …\nCreate with system signed origin and …\nWho this purports to be from and the number of extrinsics …\nThe state trie merkle root\nWasm binary unwrapped. If built with <code>SKIP_WASM_BUILD</code>, the …\nWe assume that ~5% of the block weight is consumed by …\nThe BABE epoch configuration at genesis.\nHow many parachain blocks are processed by the relay chain …\nTime and blocks.\nWe allow for 0.5 seconds of compute with a 6 second …\nThis determines the average expected block time that we …\nMoney matters.\nWe allow <code>Normal</code> extrinsics to fill up the block up to 75%, …\nA fixed point representation of a number in the range [0, 1…\nA fixed point representation of a number in the range [0, 1…\nRelay chain slot duration, in milliseconds.\nFee-related. The block saturation level. Fees will be …\nMaximum number of blocks simultaneously accepted by the …\nSee <code>PerThing::deconstruct</code>.\nConsume self and return the number of parts per thing.\nConsume self and return the number of parts per thing.\nSee <code>PerThing::deconstruct</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nSee <code>PerThing::from_float</code>.\nNOTE: saturate to 0 or 1 if x is beyond <code>[0, 1]</code>\nSee <code>PerThing::from_float</code>.\nNOTE: saturate to 0 or 1 if x is beyond <code>[0, 1]</code>\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nFrom an explicitly defined number of parts per maximum of …\nBuild this type from a number of parts per thing.\nBuild this type from a number of parts per thing.\nFrom an explicitly defined number of parts per maximum of …\nConverts a percent into <code>Self</code>. Equal to <code>x / 100</code>.\nConverts a percent into <code>Self</code>. Equal to <code>x / 100</code>.\nConverts a percent into <code>Self</code>. Equal to <code>x / 1000</code>.\nConverts a percent into <code>Self</code>. Equal to <code>x / 1000</code>.\nSee <code>PerThing::from_rational</code>.\nSee <code>PerThing::from_rational</code>.\nSee <code>PerThing::from_rational</code>.\nSee <code>PerThing::from_rational</code>.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nReturns the value of this parameter type.\nInteger division with another value, rounding down.\nInteger division with another value, rounding down.\nInteger multiplication with another value, saturating at 1.\nInteger multiplication with another value, saturating at 1.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSee <code>PerThing::is_one</code>.\nSee <code>PerThing::is_one</code>.\nSee <code>PerThing::is_zero</code>.\nSee <code>PerThing::is_zero</code>.\nSee <code>PerThing::mul_ceil</code>.\nSee <code>PerThing::mul_ceil</code>.\nSee <code>PerThing::mul_floor</code>.\nSee <code>PerThing::mul_floor</code>.\nSee <code>PerThing::one</code>\nSee <code>PerThing::one</code>\nSaturating addition. Compute <code>self + rhs</code>, saturating at the …\nSaturating addition. Compute <code>self + rhs</code>, saturating at the …\nSaturating division. Compute <code>self / rhs</code>, saturating at one …\nSaturating division. Compute <code>self / rhs</code>, saturating at one …\nSaturating multiply. Compute <code>self * rhs</code>, saturating at the …\nSaturating multiply. Compute <code>self * rhs</code>, saturating at the …\nSaturating exponentiation. Computes <code>self.pow(exp)</code>, …\nSaturating exponentiation. Computes <code>self.pow(exp)</code>, …\nSee <code>PerThing::saturating_reciprocal_mul</code>.\nSee <code>PerThing::saturating_reciprocal_mul</code>.\nSee <code>PerThing::saturating_reciprocal_mul_ceil</code>.\nSee <code>PerThing::saturating_reciprocal_mul_ceil</code>.\nSee <code>PerThing::saturating_reciprocal_mul_floor</code>.\nSee <code>PerThing::saturating_reciprocal_mul_floor</code>.\nSaturating subtraction. Compute <code>self - rhs</code>, saturating at …\nSaturating subtraction. Compute <code>self - rhs</code>, saturating at …\nSee <code>PerThing::square</code>.\nSee <code>PerThing::square</code>.\nSee <code>PerThing::zero</code>.\nSee <code>PerThing::zero</code>.")