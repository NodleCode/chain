#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchError;
use num_bigint::{BigUint, ToBigUint};
use sp_std::prelude::*;

use primitives::{CurrencyId, DerivativeIndex, PersistedValidationData, PriceDetail, Rate};

pub mod ump;
pub mod xcm;

pub trait EmergencyCallFilter<Call> {
    fn contains(call: &Call) -> bool;
}

pub trait PriceFeeder {
    fn get_price(asset_id: &CurrencyId) -> Option<PriceDetail>;
}

pub trait DecimalProvider<CurrencyId> {
    fn get_decimal(asset_id: &CurrencyId) -> Option<u8>;
}

pub trait EmergencyPriceFeeder<CurrencyId, Price> {
    fn set_emergency_price(asset_id: CurrencyId, price: Price);
    fn reset_emergency_price(asset_id: CurrencyId);
}

pub trait ExchangeRateProvider {
    fn get_exchange_rate() -> Rate;
}

pub trait LiquidStakingConvert<Balance> {
    fn staking_to_liquid(amount: Balance) -> Option<Balance>;
    fn liquid_to_staking(liquid_amount: Balance) -> Option<Balance>;
}

pub trait LiquidStakingCurrenciesProvider<CurrencyId> {
    fn get_staking_currency() -> Option<CurrencyId>;
    fn get_liquid_currency() -> Option<CurrencyId>;
}

/// Exported traits from our AMM pallet. These functions are to be used
/// by the router to enable multi route token swaps
pub trait AMM<AccountId, CurrencyId, Balance> {
    /// Based on the path specified and the available pool balances
    /// this will return the amounts outs when trading the specified
    /// amount in
    fn get_amounts_out(
        amount_in: Balance,
        path: Vec<CurrencyId>,
    ) -> Result<Vec<Balance>, DispatchError>;

    /// Based on the path specified and the available pool balances
    /// this will return the amounts in needed to produce the specified
    /// amount out
    fn get_amounts_in(
        amount_out: Balance,
        path: Vec<CurrencyId>,
    ) -> Result<Vec<Balance>, DispatchError>;

    /// Handles a "swap" on the AMM side for "who".
    /// This will move the `amount_in` funds to the AMM PalletId,
    /// trade `pair.0` to `pair.1` and return a result with the amount
    /// of currency that was sent back to the user.
    fn swap(
        who: &AccountId,
        pair: (CurrencyId, CurrencyId),
        amount_in: Balance,
    ) -> Result<(), DispatchError>;

    fn get_pools() -> Result<Vec<(CurrencyId, CurrencyId)>, DispatchError>;
}

/// Exported traits from StableSwap pallet. These functions are to be used
/// by the router.
pub trait StableSwap<AccountId, CurrencyId, Balance> {
    /// Based on the path specified and the available pool balances
    /// this will return the amounts outs when trading the specified
    /// amount in
    fn get_amounts_out(
        amount_in: Balance,
        path: Vec<CurrencyId>,
    ) -> Result<Vec<Balance>, DispatchError>;

    /// Based on the path specified and the available pool balances
    /// this will return the amounts in needed to produce the specified
    /// amount out
    fn get_amounts_in(
        amount_out: Balance,
        path: Vec<CurrencyId>,
    ) -> Result<Vec<Balance>, DispatchError>;

    /// Handles a "swap" on the AMM side for "who".
    /// This will move the `amount_in` funds to the AMM PalletId,
    /// trade `pair.0` to `pair.1` and return a result with the amount
    /// of currency that was sent back to the user.
    fn swap(
        who: &AccountId,
        pair: (CurrencyId, CurrencyId),
        amount_in: Balance,
    ) -> Result<(), DispatchError>;

    fn get_pools() -> Result<Vec<(CurrencyId, CurrencyId)>, DispatchError>;

    fn get_reserves(
        asset_in: CurrencyId,
        asset_out: CurrencyId,
    ) -> Result<(Balance, Balance), DispatchError>;
}

pub trait ConvertToBigUint {
    fn get_big_uint(&self) -> BigUint;
}

impl ConvertToBigUint for u128 {
    fn get_big_uint(&self) -> BigUint {
        self.to_biguint().unwrap()
    }
}

/// Get relaychain validation data
pub trait ValidationDataProvider {
    fn validation_data() -> Option<PersistedValidationData>;
}

/// Distribute liquidstaking asset to multi-accounts
pub trait DistributionStrategy<Balance> {
    fn get_bond_distributions(
        total_bonded_amounts: Vec<(DerivativeIndex, Balance)>,
        input: Balance,
        cap: Balance,
        min_nominator_bond: Balance,
    ) -> Vec<(DerivativeIndex, Balance)>;
    fn get_unbond_distributions(
        active_bonded_amounts: Vec<(DerivativeIndex, Balance)>,
        input: Balance,
        min_nominator_bond: Balance,
    ) -> Vec<(DerivativeIndex, Balance)>;
    fn get_rebond_distributions(
        unbonding_amounts: Vec<(DerivativeIndex, Balance)>,
        input: Balance,
    ) -> Vec<(DerivativeIndex, Balance)>;
}
