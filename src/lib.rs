#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;

use frame_support::traits::Currency;
use sp_std::prelude::*;

pub use pallet::*;
pub use weights::WeightInfo;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::ExistenceRequirement::AllowDeath;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The currency trait.
        type Currency: Currency<Self::AccountId>;

        /// The maximum number of streams per account.
        #[pallet::constant]
        type MaxStreams: Get<u32>;

        /// Information on runtime weights.
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new stream was successfully opened. [source, target, spend_rate]
        StreamOpened(T::AccountId, T::AccountId, BalanceOf<T>),
        /// A stream was successfully closed. [source, target, spend_rate]
        StreamClosed(T::AccountId, T::AccountId, BalanceOf<T>),
        /// A payment was made by a stream. [source, target, amount]
        PaymentMade(T::AccountId, T::AccountId, BalanceOf<T>),
        /// A payment failed [source, target, amount, reason]
        PaymentFailed(T::AccountId, T::AccountId, BalanceOf<T>, DispatchError),
    }

    /// Error for the stream-payments pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// A new stream cannot be opened because the maximum number of
        /// streams for source account was already reached.
        StreamLimitReached,
        /// Cannot create a stream with the target being the same account as the source.
        ReflexiveStream,
        /// Stream with given origin/index does not exist.
        StreamNotFound,
    }

    #[derive(
        Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo,
    )]
    pub struct Stream<AccountId, Balance> {
        pub target: AccountId,
        pub spend_rate: Balance,
    }

    /// The lookup table for streams.
    #[pallet::storage]
    #[pallet::getter(fn streams)]
    pub(super) type Streams<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<Stream<AccountIdOf<T>, BalanceOf<T>>, T::MaxStreams>,
        ValueQuery,
    >;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            let mut i: u32 = 0;
            for (origin, streams) in <Streams<T>>::iter() {
                for Stream { target, spend_rate } in streams.iter() {
                    i += 1;
                    if let Err(e) = T::Currency::transfer(&origin, target, *spend_rate, AllowDeath)
                    {
                        Self::deposit_event(Event::PaymentFailed(
                            origin.clone(),
                            target.clone(),
                            *spend_rate,
                            e,
                        ));
                    } else {
                        Self::deposit_event(Event::PaymentMade(
                            origin.clone(),
                            target.clone(),
                            *spend_rate,
                        ));
                    }
                }
            }
            <T as Config>::WeightInfo::on_initialize(i)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(<T as Config>::WeightInfo::open_stream())]
        pub fn open_stream(
            origin: OriginFor<T>,
            target: AccountIdOf<T>,
            spend_rate: BalanceOf<T>,
        ) -> DispatchResult {
            let source = ensure_signed(origin)?;
            if source == target {
                return Err(Error::<T>::ReflexiveStream.into());
            }
            <Streams<T>>::try_mutate(&source, |streams| {
                streams.try_push(Stream {
                    target: target.clone(),
                    spend_rate,
                })
            })
            .map_err(|_| Error::<T>::StreamLimitReached)?;
            Self::deposit_event(Event::StreamOpened(source, target, spend_rate));
            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::close_stream(0))]
        pub fn close_stream(origin: OriginFor<T>, index: u32) -> DispatchResult {
            let source = ensure_signed(origin)?;
            let index = index as usize;
            let Stream { target, spend_rate } = <Streams<T>>::try_mutate(&source, |streams| {
                if index < streams.len() {
                    Ok(streams.remove(index))
                } else {
                    Err(Error::<T>::StreamNotFound)
                }
            })?;
            Self::deposit_event(Event::StreamClosed(source, target, spend_rate));
            Ok(())
        }
    }
}
