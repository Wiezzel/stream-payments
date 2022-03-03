//! Benchmarking setup for pallet-template

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as StreamPayments;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::{Get, OnInitialize};
use frame_system::RawOrigin;

const SEED: u32 = 609;

fn open_n_streams<T: Config>(n: u32) -> Result<(), &'static str> {
    let caller: T::AccountId = whitelisted_caller();
    let spend_rate: BalanceOf<T> = 10u32.into();
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
        let spend_rate: BalanceOf<T> = 10u32.into();
    }: _(RawOrigin::Signed(caller), target, spend_rate)

    // Fill in the streams vector for a single account up to the max and then remove
    // a single stream from every position. It's imperfect because it relies on MaxStreams,
    // but still better than doing just one case.
    close_stream {
        let i in 0..(T::MaxStreams::get() - 1);  // Range end seems to be **inclusive** (ugh!)
        open_n_streams::<T>(T::MaxStreams::get())?;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), i)

    on_initialize {
        let i in 0..(T::MaxStreams::get() - 1);  // Range end seems to be **inclusive** (ugh!)
        open_n_streams::<T>(i)?;
    } : { StreamPayments::<T>::on_initialize(2u32.into()); }

    impl_benchmark_test_suite!(StreamPayments, crate::mock::new_test_ext(), crate::mock::Test);
}
