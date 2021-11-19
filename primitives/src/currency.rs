/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use frame_support::traits::fungibles::{Inspect, Mutate};
use sp_runtime::{traits::Convert, SaturatedConversion};
use sp_std::{convert::Into, marker::PhantomData, prelude::*, result};
use xcm::latest::prelude::*;
use xcm_executor::traits::{Convert as MoreConvert, MatchesFungible, TransactAsset};

pub struct MultiCurrencyAdapter<
    MultiCurrency,
    Match,
    AccountId,
    AccountIdConvert,
    CurrencyIdConvert,
>(
    PhantomData<(
        MultiCurrency,
        Match,
        AccountId,
        AccountIdConvert,
        CurrencyIdConvert,
    )>,
);

enum Error {
    /// Failed to match fungible.
    FailedToMatchFungible,
    /// `MultiLocation` to `AccountId` Conversion failed.
    AccountIdConversionFailed,
    /// `CurrencyId` conversion failed.
    CurrencyIdConversionFailed,
}

impl From<Error> for XcmError {
    fn from(e: Error) -> Self {
        match e {
            Error::FailedToMatchFungible => {
                XcmError::FailedToTransactAsset("FailedToMatchFungible")
            }
            Error::AccountIdConversionFailed => {
                XcmError::FailedToTransactAsset("AccountIdConversionFailed")
            }
            Error::CurrencyIdConversionFailed => {
                XcmError::FailedToTransactAsset("CurrencyIdConversionFailed")
            }
        }
    }
}

impl<
        MultiCurrency: Inspect<AccountId> + Mutate<AccountId>,
        Match: MatchesFungible<MultiCurrency::Balance>,
        AccountId: sp_std::fmt::Debug + Clone,
        AccountIdConvert: MoreConvert<MultiLocation, AccountId>,
        CurrencyIdConvert: Convert<MultiAsset, Option<MultiCurrency::AssetId>>,
    > TransactAsset
    for MultiCurrencyAdapter<MultiCurrency, Match, AccountId, AccountIdConvert, CurrencyIdConvert>
{
    fn deposit_asset(asset: &MultiAsset, location: &MultiLocation) -> XcmResult {
        match (
            AccountIdConvert::convert_ref(location),
            CurrencyIdConvert::convert(asset.clone()),
            Match::matches_fungible(asset),
        ) {
            // known asset
            (Ok(who), Some(currency_id), Some(amount)) => {
                MultiCurrency::mint_into(currency_id, &who, amount)
                    .map_err(|e| XcmError::FailedToTransactAsset(e.into()))
            }
            // ignore unknown asset
            _ => Ok(()),
        }
    }

    fn withdraw_asset(
        asset: &MultiAsset,
        location: &MultiLocation,
    ) -> result::Result<xcm_executor::Assets, XcmError> {
        let who = AccountIdConvert::convert_ref(location)
            .map_err(|_| XcmError::from(Error::AccountIdConversionFailed))?;
        let currency_id = CurrencyIdConvert::convert(asset.clone())
            .ok_or_else(|| XcmError::from(Error::CurrencyIdConversionFailed))?;
        let amount: MultiCurrency::Balance = Match::matches_fungible(asset)
            .ok_or_else(|| XcmError::from(Error::FailedToMatchFungible))?
            .saturated_into();
        MultiCurrency::burn_from(currency_id, &who, amount)
            .map_err(|e| XcmError::FailedToTransactAsset(e.into()))?;

        Ok(asset.clone().into())
    }
}
