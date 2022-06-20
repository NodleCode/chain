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
use codec::{Decode, Encode};
use derivative::Derivative;
use frame_support::{pallet_prelude::MaxEncodedLen, traits::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_std::{fmt::Debug, prelude::*};

#[derive(Derivative)]
#[derivative(
	Debug(bound = "T: Debug"),
	Clone(bound = "T: Clone"),
	PartialEq(bound = "T: PartialEq"),
	Eq(bound = "T: Eq")
)]
#[derive(Decode, Encode, TypeInfo)]
#[scale_info(skip_type_params(S))]
pub struct OrderedSet<T, S: Get<u32>>(pub BoundedVec<T, S>);

impl<T, S> MaxEncodedLen for OrderedSet<T, S>
where
	T: Encode + Decode,
	S: Get<u32>,
{
	fn max_encoded_len() -> usize {
		S::get() as usize
	}
}

impl<T, S> Default for OrderedSet<T, S>
where
	T: Encode + Decode,
	S: Get<u32>,
{
	/// Create a default empty set
	fn default() -> Self {
		let inner: Vec<T> = Vec::with_capacity(S::get() as usize);
		Self(BoundedVec::try_from(inner).expect("OrderedSet Failed To Create Default"))
	}
}

impl<T, S> OrderedSet<T, S>
where
	T: Ord + Encode + Decode + Clone,
	S: Get<u32>,
{
	/// Create a new empty set
	pub fn new() -> Self {
		Self::default()
	}

	/// Create a set from a `Vec`.
	/// `v` will be sorted and dedup first.
	pub fn from(mut inner: Vec<T>) -> Result<Self, ()> {
		inner.sort();
		inner.dedup();
		Self::from_sorted_set(inner)
	}

	/// Create a set from a `Vec`.
	/// Assume `v` is sorted and contain unique elements.
	pub fn from_sorted_set(val: Vec<T>) -> Result<Self, ()> {
		let inner = BoundedVec::try_from(val).map_err(|_| {
			log::error!("OrderedSet Failed To Create From Vec");
			()
		})?;
		Ok(Self(inner))
	}

	pub fn get_inner(&self) -> Result<Vec<T>, ()> {
		let inner = self.0.clone().to_vec();
		Ok(inner.clone())
	}

	pub fn len(&self) -> Result<usize, ()> {
		let inner = self.0.clone().to_vec();
		Ok(inner.len())
	}

	pub fn update_inner(&mut self, inner: Vec<T>) -> Result<(), ()> {
		self.0 = BoundedVec::try_from(inner).map_err(|_| {
			log::error!("Orderset Failed to Update inner");
			()
		})?;
		Ok(())
	}

	/// Insert an element.
	/// Return true if insertion happened.
	pub fn insert(&mut self, value: T) -> Result<bool, ()> {
		let mut inner = self.0.clone().to_vec();

		match inner.binary_search(&value) {
			Ok(_) => Ok(false),
			Err(loc) => {
				inner.insert(loc, value);
				self.0 = BoundedVec::try_from(inner).map_err(|_| {
					log::error!("Orderset Failed to Insert");
					()
				})?;
				Ok(true)
			}
		}
	}

	/// Remove an element.
	/// Return true if removal happened.
	pub fn remove(&mut self, value: &T) -> Result<bool, ()> {
		let mut inner = self.0.clone().to_vec();

		match inner.binary_search(&value) {
			Ok(loc) => {
				inner.remove(loc);
				self.0 = BoundedVec::try_from(inner).map_err(|_| {
					log::error!("Orderset Failed to Remove");
					()
				})?;
				Ok(true)
			}
			Err(_) => Ok(false),
		}
	}

	/// Return if the set contains `value`
	pub fn contains(&self, value: &T) -> Result<usize, ()> {
		let inner = self.0.clone().to_vec();

		match inner.binary_search(&value) {
			Ok(loc) => Ok(loc),
			Err(_) => Err(()),
		}
	}

	/// Clear the set
	pub fn clear(&mut self) -> Result<(), ()> {
		let mut inner = self.0.clone().to_vec();

		inner.clear();
		self.0 = BoundedVec::try_from(inner).map_err(|_| {
			log::error!("Orderset Failed to Remove");
			()
		})?;

		Ok(())
	}
}

impl<T, S> TryFrom<Vec<T>> for OrderedSet<T, S>
where
	T: Ord + Encode + Decode + Clone,
	S: Get<u32>,
{
	type Error = ();
	fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
		let rval = Self::from(v).map_err(|_| ())?;
		Ok(rval)
	}
}
