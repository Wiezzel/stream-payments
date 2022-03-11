//! Benchmarking setup for pallet-template

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as StreamPayments;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::{Get, OnInitialize};
use frame_system::RawOrigin;

const SEED: u32 = 609;

fn open_n_streams<T: Config, V: Into<BalanceOf<T>>>(
    n: u32,
    spend_rate: V,
) -> Result<(), &'static str> {
    let caller: T::AccountId = whitelisted_caller();
    T::Currency::make_free_balance_be(&caller, 1_000_000_000u32.into());
    let spend_rate: BalanceOf<T> = spend_rate.into();
    for i in 0..n {
        let target: T::AccountId = account("target", i, SEED);
        StreamPayments::<T>::open_stream(
            RawOrigin::Signed(caller.clone()).into(),
            target,
            spend_rate,
        )?;
    }
    Ok(())
}

benchmarks! {
    open_stream {
        let caller: T::AccountId = whitelisted_caller();
        let target: T::AccountId = account("target", 1, SEED);
        T::Currency::make_free_balance_be(&caller, 1_000_000_000u32.into());
        let spend_rate: BalanceOf<T> = 10u32.into();
    }: _(RawOrigin::Signed(caller.clone()), target, spend_rate)
    verify {
        assert_eq!(StreamPayments::<T>::streams(caller).len(), 1u32 as usize);
    }

    // Fill in the streams vector for a single account up to the max and then remove
    // a single stream from every position. It's imperfect because it relies on MaxStreams,
    // but still better than doing just one case.
    close_stream {
        let i in 0..(T::MaxStreams::get() - 1);  // Range end seems to be **inclusive** (ugh!)
        open_n_streams::<T, u32>(T::MaxStreams::get(), 1000)?;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), i)
    verify {
        assert_eq!(StreamPayments::<T>::streams(caller).len(), (T::MaxStreams::get() - 1) as usize);
    }

    on_initialize_transfer {
        let i in 0..(T::MaxStreams::get() - 1);  // Range end seems to be **inclusive** (ugh!)
        open_n_streams::<T, u32>(i, 1000)?;
        let caller: T::AccountId = whitelisted_caller();
    } : {
        StreamPayments::<T>::on_initialize(2u32.into());
    } verify {
        assert_eq!(StreamPayments::<T>::streams(caller).len(), i as usize);
    }

    on_initialize_stream_exhausted {
        let i in 0..(T::MaxStreams::get() - 1);  // Range end seems to be **inclusive** (ugh!)
        // Spend rate has to be bigger than (i * deposit) so that total amount of freed deposits
        // does not suffice to make any payments.
        let spend_rate = T::StreamDeposit::get() * i.into() + 1u32.into();
        open_n_streams::<T, BalanceOf<T>>(i, spend_rate)?;
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, 0u32.into());
    } : {
        StreamPayments::<T>::on_initialize(2u32.into());
    } verify {
        assert_eq!(*StreamPayments::<T>::streams(caller), []);
    }

    impl_benchmark_test_suite!(StreamPayments, crate::mock::new_test_ext(), crate::mock::Test);
}
