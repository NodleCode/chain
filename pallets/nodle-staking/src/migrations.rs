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
use super::{BalanceOf, Config};
use crate::types::Validator;
use frame_support::{
    pallet_prelude::*,
    traits::{Get, StorageInstance},
    weights::{constants::RocksDbWeight, Weight},
};

use sp_runtime::{traits::Zero, Perbill};

use sp_std::vec::Vec;

/// This migration is in charge of moving the validators from POA to Staking pallet to
/// work with the last chain reboot.
/// We have to do two things:
/// - read "Validators" storage instance from POA.
/// - with read info update the "Invulnerables" & "ValidatorState" of staking pallet.
pub fn poa_validators_migration<T: Config>() -> Weight {
    // Buffer to account for misc checks
    let mut weight: Weight = 1_000;

    log::info!("🕊️ Starting Poa validators migration...");

    struct POAValidatorsStorageValuePrefix;
    impl StorageInstance for POAValidatorsStorageValuePrefix {
        const STORAGE_PREFIX: &'static str = "Validators";
        fn pallet_prefix() -> &'static str {
            "Poa"
        }
    }

    struct StakingValidatorStateStorageMapPrefix;
    impl StorageInstance for StakingValidatorStateStorageMapPrefix {
        const STORAGE_PREFIX: &'static str = "ValidatorState";
        fn pallet_prefix() -> &'static str {
            "NodleStaking"
        }
    }

    struct StakingInvulnerablesStorageValuePrefix;
    impl StorageInstance for StakingInvulnerablesStorageValuePrefix {
        const STORAGE_PREFIX: &'static str = "Invulnerables";
        fn pallet_prefix() -> &'static str {
            "NodleStaking"
        }
    }

    struct StakingTotalSelectedStorageValuePrefix;
    impl StorageInstance for StakingTotalSelectedStorageValuePrefix {
        const STORAGE_PREFIX: &'static str = "TotalSelected";
        fn pallet_prefix() -> &'static str {
            "NodleStaking"
        }
    }

    struct StakingValidatorFeeStorageValuePrefix;
    impl StorageInstance for StakingValidatorFeeStorageValuePrefix {
        const STORAGE_PREFIX: &'static str = "ValidatorFee";
        fn pallet_prefix() -> &'static str {
            "NodleStaking"
        }
    }

    struct StakingSlashRewardProportionStorageValuePrefix;
    impl StorageInstance for StakingSlashRewardProportionStorageValuePrefix {
        const STORAGE_PREFIX: &'static str = "SlashRewardProportion";
        fn pallet_prefix() -> &'static str {
            "NodleStaking"
        }
    }

    #[allow(type_alias_bounds)]
    type POAValidators<T: Config> =
        StorageValue<POAValidatorsStorageValuePrefix, Vec<T::AccountId>, OptionQuery>;

    #[allow(type_alias_bounds)]
    type StakingTotalSelected =
        StorageValue<StakingTotalSelectedStorageValuePrefix, u32, ValueQuery>;

    #[allow(type_alias_bounds)]
    type StakingValidatorFee =
        StorageValue<StakingValidatorFeeStorageValuePrefix, Perbill, ValueQuery>;

    #[allow(type_alias_bounds)]
    type StakingSlashRewardProportion =
        StorageValue<StakingSlashRewardProportionStorageValuePrefix, Perbill, ValueQuery>;

    #[allow(type_alias_bounds)]
    type StakingInvulnerables<T: Config> =
        StorageValue<StakingInvulnerablesStorageValuePrefix, Vec<T::AccountId>, OptionQuery>;

    #[allow(type_alias_bounds)]
    type StakingValidatorState<T: Config> = StorageMap<
        StakingValidatorStateStorageMapPrefix,
        Twox64Concat,
        T::AccountId,
        Validator<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    log::info!(
        "<POAValidators<T>>::exits()? {:?}",
        <POAValidators<T>>::exists()
    );
    weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 0));

    log::info!(
        "<StakingInvulnerables<T>>::exits()? {:?}",
        <StakingInvulnerables<T>>::exists()
    );
    weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 0));

    if !<POAValidators<T>>::exists() {
        weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 0));
        log::info!("🕊️ POA Validators doesn't exist Nothing to migrate!!!");
    } else {
        if let Some(poa_validators) = <POAValidators<T>>::get() {
            weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 0));

            if let Some(pre_invulnerable_validators) = <StakingInvulnerables<T>>::get() {
                weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 0));

                let mut post_invulnerable_validators = pre_invulnerable_validators
                    .clone()
                    .iter()
                    .chain(poa_validators.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<T::AccountId>>();

                post_invulnerable_validators.sort();
                post_invulnerable_validators.dedup();

                <StakingInvulnerables<T>>::put(post_invulnerable_validators.clone());
                weight = weight.saturating_add(RocksDbWeight::get().reads_writes(0, 1));
            } else {
                log::info!("Staking runtime upgrade Genesis config");

                <StakingInvulnerables<T>>::put(poa_validators.clone());

                // Set collator commission to default config
                StakingValidatorFee::put(T::DefaultValidatorFee::get());
                // Set total selected validators to minimum config
                StakingTotalSelected::put(T::MinSelectedValidators::get());
                // Set default slash reward fraction
                StakingSlashRewardProportion::put(T::DefaultSlashRewardProportion::get());

                weight = weight.saturating_add(RocksDbWeight::get().reads_writes(0, 5));
            }

            for valid_acc in &poa_validators {
                if <StakingValidatorState<T>>::contains_key(&valid_acc) {
                    weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 0));

                    log::trace!(
                    "poa_validators_migration>[{:#?}]=> - Already staker Ignoring Address -[{:#?}]",
                    line!(),
                    valid_acc
                );
                } else {
                    log::trace!(
                        "poa_validators_migration>[{:#?}]=> - Adding Address -[{:#?}]",
                        line!(),
                        valid_acc
                    );

                    <StakingValidatorState<T>>::insert(
                        &valid_acc,
                        Validator::<T::AccountId, BalanceOf<T>>::new(
                            valid_acc.clone(),
                            Zero::zero(),
                        ),
                    );

                    log::trace!(
                        "poa_validators_migration>[{:#?}]=> - Address Added-[{:#?}]",
                        line!(),
                        <StakingValidatorState<T>>::contains_key(&valid_acc),
                    );

                    weight = weight.saturating_add(RocksDbWeight::get().reads_writes(0, 1));
                }
            }

            <POAValidators<T>>::kill();

            log::info!(
                "🕊️ Sucess!!! POA Validators of len {:#?} moved to Staking pallet",
                poa_validators.len()
            );
        } else {
            log::warn!("🕊️ Warning!!! Unable to read POA Validators instance");
        }
    }

    weight
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock;
    use crate::mock::{events, AccountId, ExtBuilder, NodleStaking, Origin, Poa};
    use crate::types;
    use frame_support::assert_ok;
    use frame_support::traits::InitializeMembers;

    use crate as nodle_staking;

    #[test]
    fn test_empty_list_on_migration_works() {
        ExtBuilder::default().build_and_execute(|| {
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

            NodleStaking::on_runtime_upgrade();

            assert_eq!(Poa::validators().len(), 0);
            assert_eq!(NodleStaking::invulnerables().len(), 0);
        });
    }

    #[test]
    fn test_valid_list_on_migration_works() {
        ExtBuilder::default().build_and_execute(|| {
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

            NodleStaking::on_runtime_upgrade();

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
