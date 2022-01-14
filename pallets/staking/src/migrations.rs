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
use sp_runtime::traits::Zero;
use sp_std::vec::Vec;

pub mod v1 {
    use super::*;

    use crate::{
        types::Validator, BalanceOf, Config, Invulnerables, Pallet, SlashRewardProportion,
        StakingMaxValidators, StakingMinNominationChillThreshold, StakingMinNominatorTotalBond,
        StakingMinStakeSessionSelection, StakingMinValidatorBond, TotalSelected, ValidatorFee,
        ValidatorState,
    };
    use frame_support::{
        generate_storage_alias,
        pallet_prelude::*,
        traits::{Get, OnRuntimeUpgrade, StorageVersion},
        weights::{constants::RocksDbWeight, Weight},
    };

    /// The current storage version.
    pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    generate_storage_alias!(
        Poa, Validators<T: Config> => Value<
            Vec<T::AccountId>,
            OptionQuery
        >
    );

    pub struct PoAToStaking<T>(PhantomData<T>);
    impl<T: Config> OnRuntimeUpgrade for PoAToStaking<T> {
        fn on_runtime_upgrade() -> Weight {
            let mut weight: Weight = 1_000;
            log::info!("Starting Poa to Staking migration...");

            let storage_version = StorageVersion::get::<Pallet<T>>();

            log::info!(
                "on_runtime_upgrade>[{:#?}]=> - Storage Version Current-[{:#?}], New-[{:#?}]",
                line!(),
                storage_version,
                STORAGE_VERSION
            );

            // Check for storage version
            if storage_version >= STORAGE_VERSION {
                return weight;
            }

            if let Some(poa_validators) = <Validators<T>>::get() {
                weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 0));

                let pre_invulnerable_validators = <Invulnerables<T>>::get();
                if !pre_invulnerable_validators.is_empty() {
                    weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 0));

                    let mut post_invulnerable_validators = pre_invulnerable_validators
                        .clone()
                        .iter()
                        .chain(poa_validators.iter())
                        .map(|x| x.clone())
                        .collect::<Vec<T::AccountId>>();

                    post_invulnerable_validators.sort();
                    post_invulnerable_validators.dedup();

                    <Invulnerables<T>>::put(post_invulnerable_validators.clone());
                    weight = weight.saturating_add(RocksDbWeight::get().reads_writes(0, 1));
                } else {
                    log::info!("Staking runtime upgrade Genesis config");

                    <Invulnerables<T>>::put(poa_validators.clone());

                    // Set collator commission to default config
                    <ValidatorFee<T>>::put(T::DefaultValidatorFee::get());
                    // Set total selected validators to minimum config
                    <TotalSelected<T>>::put(T::MinSelectedValidators::get());
                    // Set default slash reward fraction
                    <SlashRewardProportion<T>>::put(T::DefaultSlashRewardProportion::get());

                    <StakingMaxValidators<T>>::put(T::DefaultStakingMaxValidators::get());
                    <StakingMinStakeSessionSelection<T>>::put(
                        T::DefaultStakingMinStakeSessionSelection::get(),
                    );
                    <StakingMinValidatorBond<T>>::put(T::DefaultStakingMinValidatorBond::get());
                    <StakingMinNominationChillThreshold<T>>::put(
                        T::DefaultStakingMinNominationChillThreshold::get(),
                    );
                    <StakingMinNominatorTotalBond<T>>::put(
                        T::DefaultStakingMinNominatorTotalBond::get(),
                    );

                    weight = weight.saturating_add(RocksDbWeight::get().reads_writes(0, 5));
                }

                for valid_acc in &poa_validators {
                    if <ValidatorState<T>>::contains_key(&valid_acc) {
                        weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 0));
                        log::trace!(
							"on_runtime_upgrade>[{:#?}]=> - Already staker Ignoring Address -[{:#?}]",
							line!(),
							valid_acc
						);
                    } else {
                        log::trace!(
                            "on_runtime_upgrade>[{:#?}]=> - Adding Address -[{:#?}]",
                            line!(),
                            valid_acc
                        );

                        <ValidatorState<T>>::insert(
                            &valid_acc,
                            Validator::<T::AccountId, BalanceOf<T>>::new(
                                valid_acc.clone(),
                                Zero::zero(),
                            ),
                        );

                        log::trace!(
                            "on_runtime_upgrade>[{:#?}]=> - Address Added-[{:#?}]",
                            line!(),
                            <ValidatorState<T>>::contains_key(&valid_acc),
                        );

                        weight = weight.saturating_add(RocksDbWeight::get().reads_writes(0, 1));
                    }
                }

                <Validators<T>>::kill();

                STORAGE_VERSION.put::<Pallet<T>>();

                log::info!(
                    "on_runtime_upgrade>[{:#?}]=>Sucess!!! POA Validators of len {:#?} moved to Staking pallet",
					line!(),
					poa_validators.len()
                );
            }

            let storage_version = StorageVersion::get::<Pallet<T>>();

            log::info!(
                "on_runtime_upgrade>[{:#?}]=> - Storage Version Current-[{:#?}], New-[{:#?}]",
                line!(),
                storage_version,
                STORAGE_VERSION
            );

            weight
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<(), &'static str> {
            log::info!(
                "pre_upgrade[{:#?}]=>Validators.exits()? {:?}",
                line!(),
                <Validators<T>>::get().is_some()
            );

            let storage_version = StorageVersion::get::<Pallet<T>>();

            log::info!(
                "pre_upgrade>[{:#?}]=> - Storage Version Current-[{:#?}], New-[{:#?}]",
                line!(),
                storage_version,
                STORAGE_VERSION
            );

            // these must exist.
            assert!(
                <Validators<T>>::get().is_some(),
                "Poa Validators storage item not found!"
            );
            Ok(())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade() -> Result<(), &'static str> {
            log::info!(
                "post_upgrade[{:#?}]=>Validators.exits()? {:?}",
                line!(),
                <Validators<T>>::get().is_some()
            );

            let storage_version = StorageVersion::get::<Pallet<T>>();

            log::info!(
                "post_upgrade>[{:#?}]=> - Storage Version Current-[{:#?}], New-[{:#?}]",
                line!(),
                storage_version,
                STORAGE_VERSION
            );

            // should not exist.
            assert!(
                !<Validators<T>>::get().is_some(),
                "Poa Validators storage item not cleaned up!"
            );
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrations;
    use crate::mock;
    use crate::mock::{events, AccountId, ExtBuilder, NodleStaking, Origin, Poa, Test};
    use crate::types;
    use frame_support::assert_ok;
    use frame_support::traits::InitializeMembers;
    use frame_support::traits::OnRuntimeUpgrade;

    use crate as nodle_staking;

    #[test]
    #[cfg(not(tarpaulin))]
    fn test_empty_list_on_migration_works() {
        ExtBuilder::default()
            .num_validators(4)
            .build_and_execute(|| {
                mock::start_active_session(1);

                let expected = vec![
                    nodle_staking::Event::ValidatorChosen(2, 11, 1500),
                    nodle_staking::Event::ValidatorChosen(2, 21, 1000),
                    nodle_staking::Event::ValidatorChosen(2, 41, 1000),
                    nodle_staking::Event::NewSession(5, 2, 3, 3500),
                ];
                assert_eq!(events(), expected);

                assert_eq!(Poa::validators().len(), 0);
                assert_eq!(NodleStaking::invulnerables().len(), 0);

                // #[cfg(feature = "try-runtime")]
                // assert_ok!(migrations::v1::PoAToStaking::<Test>::pre_upgrade());

                migrations::v1::PoAToStaking::<Test>::on_runtime_upgrade();

                // #[cfg(feature = "try-runtime")]
                // assert_ok!(migrations::v1::PoAToStaking::<Test>::post_upgrade());

                assert_eq!(Poa::validators().len(), 0);
                assert_eq!(NodleStaking::invulnerables().len(), 0);
            });
    }

    #[test]
    fn test_valid_list_on_migration_works() {
        ExtBuilder::default()
            .num_validators(4)
            .build_and_execute(|| {
                mock::start_active_session(1);

                let mut expected = vec![
                    nodle_staking::Event::ValidatorChosen(2, 11, 1500),
                    nodle_staking::Event::ValidatorChosen(2, 21, 1000),
                    nodle_staking::Event::ValidatorChosen(2, 41, 1000),
                    nodle_staking::Event::NewSession(5, 2, 3, 3500),
                ];
                assert_eq!(events(), expected);

                assert_eq!(NodleStaking::total(), 3500);

                assert_eq!(Poa::validators().len(), 0);
                assert_eq!(NodleStaking::invulnerables().len(), 0);

                let poa_validators: Vec<AccountId> = vec![11, 21, 31, 61, 71, 81];
                Poa::initialize_members(&poa_validators);

                assert_eq!(Poa::validators(), [11, 21, 31, 61, 71, 81].to_vec());

                #[cfg(feature = "try-runtime")]
                assert_ok!(migrations::v1::PoAToStaking::<Test>::pre_upgrade());

                migrations::v1::PoAToStaking::<Test>::on_runtime_upgrade();

                #[cfg(feature = "try-runtime")]
                assert_ok!(migrations::v1::PoAToStaking::<Test>::post_upgrade());

                assert_eq!(Poa::validators().len(), 0);
                assert_eq!(
                    NodleStaking::invulnerables(),
                    [11, 21, 31, 61, 71, 81].to_vec()
                );

                assert_eq!(NodleStaking::total(), 3500);

                assert_eq!(
                    NodleStaking::validator_state(&11).unwrap().state,
                    types::ValidatorStatus::Active
                );

                assert_eq!(
                    NodleStaking::validator_state(&21).unwrap().state,
                    types::ValidatorStatus::Active
                );

                assert_eq!(
                    NodleStaking::validator_state(&31).unwrap().state,
                    types::ValidatorStatus::Active
                );

                assert_eq!(
                    NodleStaking::validator_state(&61).unwrap().state,
                    types::ValidatorStatus::Active
                );

                assert_eq!(
                    NodleStaking::validator_state(&71).unwrap().state,
                    types::ValidatorStatus::Active
                );

                assert_eq!(
                    NodleStaking::validator_state(&81).unwrap().state,
                    types::ValidatorStatus::Active
                );

                mock::start_active_session(2);

                let mut new1 = vec![
                    nodle_staking::Event::ValidatorChosen(3, 11, 1500),
                    nodle_staking::Event::ValidatorChosen(3, 21, 1000),
                    nodle_staking::Event::ValidatorChosen(3, 31, 0),
                    nodle_staking::Event::ValidatorChosen(3, 41, 1000),
                    nodle_staking::Event::ValidatorChosen(3, 61, 0),
                    nodle_staking::Event::ValidatorChosen(3, 71, 0),
                    nodle_staking::Event::ValidatorChosen(3, 81, 0),
                    nodle_staking::Event::NewSession(10, 3, 7, 3500),
                ];

                expected.append(&mut new1);
                assert_eq!(events(), expected);

                assert_ok!(NodleStaking::validator_bond_more(Origin::signed(31), 500));
                assert_ok!(NodleStaking::validator_bond_more(Origin::signed(61), 500));
                assert_ok!(NodleStaking::validator_bond_more(Origin::signed(71), 500));
                assert_ok!(NodleStaking::validator_bond_more(Origin::signed(81), 500));

                mock::start_active_session(3);

                let mut new2 = vec![
                    nodle_staking::Event::ValidatorBondedMore(31, 0, 500),
                    nodle_staking::Event::ValidatorBondedMore(61, 0, 500),
                    nodle_staking::Event::ValidatorBondedMore(71, 0, 500),
                    nodle_staking::Event::ValidatorBondedMore(81, 0, 500),
                    nodle_staking::Event::ValidatorChosen(4, 11, 1500),
                    nodle_staking::Event::ValidatorChosen(4, 21, 1000),
                    nodle_staking::Event::ValidatorChosen(4, 31, 500),
                    nodle_staking::Event::ValidatorChosen(4, 41, 1000),
                    nodle_staking::Event::ValidatorChosen(4, 61, 500),
                    nodle_staking::Event::ValidatorChosen(4, 71, 500),
                    nodle_staking::Event::ValidatorChosen(4, 81, 500),
                    nodle_staking::Event::NewSession(15, 4, 7, 5500),
                ];

                expected.append(&mut new2);
                assert_eq!(events(), expected);

                assert_eq!(NodleStaking::total(), 5500);
            });
    }
}
