/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2022  Nodle International
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

//! Root Of Trust pallet benchmarks

#![cfg(feature = "runtime-benchmarks")]
#![allow(unused)]

use super::*;

use frame_benchmarking::impl_benchmark_test_suite;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

use crate::Pallet as RootOfTrust;

const SEED_MANAGER: u32 = 0;

fn register<T: Config>(index: u32) -> Result<T::AccountId, &'static str> {
    let manager = account("manager", index, SEED_MANAGER);
    T::Currency::make_free_balance_be(&manager, BalanceOf::<T>::max_value());
    <Pallet<T>>::benchmark_set_members(&[manager.clone()]);

    Ok(manager)
}

benchmarks! {
    book_slot {
        let manager = register::<T>(0)?;
        let certificate: T::CertificateId = Default::default();
    }: _(RawOrigin::Signed(manager), certificate)

    renew_slot {
        let manager = register::<T>(0)?;
        let certificate: T::CertificateId = Default::default();

        let _ = <Pallet<T>>::book_slot(RawOrigin::Signed(manager.clone()).into(), certificate.clone());
    }: _(RawOrigin::Signed(manager), certificate)

    revoke_slot {
        let manager = register::<T>(0)?;
        let certificate: T::CertificateId = Default::default();

        let _ = <Pallet<T>>::book_slot(RawOrigin::Signed(manager.clone()).into(), certificate.clone());
    }: _(RawOrigin::Signed(manager), certificate)

    revoke_child {
        let manager = register::<T>(0)?;
        let certificate: T::CertificateId = Default::default();
        let child: T::CertificateId = Default::default();

        let _ = <Pallet<T>>::book_slot(RawOrigin::Signed(manager.clone()).into(), certificate.clone());
    }: _(RawOrigin::Signed(manager), certificate, child)

    impl_benchmark_test_suite!(
        RootOfTrust,
        crate::tests::new_test_ext(),
        crate::tests::Test,
    );
}
