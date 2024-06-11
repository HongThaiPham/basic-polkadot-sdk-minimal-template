//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
    use frame::prelude::*;
    // use frame::{pallet, traits::tokens::Balance};
    // use frame_system::Config;
    // use storage::StorageMap;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        fn ed() -> Balance;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    pub type Balance = u128;

    #[pallet::storage]
    pub type TotalIssuance<T: Config> = StorageValue<_, Balance, ValueQuery>;
    // pub type TotalIssuance<T: Config> = StorageValue<Value = Balance>;
    // add ValueQuery to get return value instead of optional

    #[pallet::storage]
    // pub type Balances<T: Config> = StorageMap<_, _, T::AccountId, Balance>;
    pub type Balances<T: Config> =
        StorageMap<Key = <T as frame_system::Config>::AccountId, Value = Balance>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn mint_unsafe(origin: OriginFor<T>, amount: Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if amount < T::ed() {
                return Err("BelowEd".into());
            }

            if Balances::<T>::contains_key(&who) {
                return Err("AldreadyExist".into());
            }

            Balances::<T>::insert(&who, amount);
            let mut issurance = TotalIssuance::<T>::get();
            issurance += amount;
            TotalIssuance::<T>::put(issurance);

            // OR
            // without ValueQuery
            // TotalIssuance::<T>::mutate(|t| *t = Some(t.unwrap_or(0) + amount));
            // with ValueQuery
            // TotalIssuance::<T>::mutate(|t| *t += amount);
            Ok(())
        }

        pub fn transfer(_origin: T::RuntimeOrigin) -> DispatchResult {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::pallet as currency_pallet;
    use super::pallet::*;
    use frame::testing_prelude::*;

    construct_runtime!(
        pub enum Runtime {
            System: frame_system,
            Currency: currency_pallet,
        }
    );

    #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
    impl frame_system::Config for Runtime {
        type Block = MockBlock<Runtime>;
        // within pallet we just said `<T as frame_system::Config>::AccountId`, now we
        // finally specified it.
        type AccountId = u64;
    }

    impl currency_pallet::Config for Runtime {
        fn ed() -> Balance {
            10
        }
    }

    #[test]
    fn mint_unsafe_works() {
        TestState::new_empty().execute_with(|| {
            // execute transaction
            assert_ok!(currency_pallet::Pallet::<Runtime>::mint_unsafe(
                RuntimeOrigin::signed(1),
                10
            ));
            assert_eq!(currency_pallet::Balances::<Runtime>::get(1), Some(10));
            assert_eq!(currency_pallet::TotalIssuance::<Runtime>::get(), 10);
        });
    }

    #[test]
    fn mint_into_existed_fails() {
        TestState::new_empty().execute_with(|| {
            assert_ok!(currency_pallet::Pallet::<Runtime>::mint_unsafe(
                RuntimeOrigin::signed(1),
                10
            ));
            assert_noop!(
                currency_pallet::Pallet::<Runtime>::mint_unsafe(RuntimeOrigin::signed(1), 10),
                "AldreadyExist"
            );
        });
    }

    #[test]
    fn mint_below_ed_fail() {
        TestState::new_empty().execute_with(|| {
            assert_noop!(
                currency_pallet::Pallet::<Runtime>::mint_unsafe(RuntimeOrigin::signed(1), 5),
                "BelowEd"
            );
        });
    }
}
