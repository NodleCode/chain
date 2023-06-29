/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
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


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_nodle_uniques.
pub trait WeightInfo {
	fn mint_with_extra_deposit() -> Weight;
	fn burn() -> Weight;
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn mint_with_extra_deposit() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn burn() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn destroy(_n: u32, _m: u32, _a: u32) -> Weight {
		Weight::from_parts(0, 0)
	}
}

impl WeightInfo for () {
	fn mint_with_extra_deposit() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn burn() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn destroy(_n: u32, _m: u32, _a: u32) -> Weight {
		Weight::from_parts(0, 0)
	}
}