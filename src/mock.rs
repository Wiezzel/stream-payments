use crate as stream_payments;
use frame_support::traits::{ConstU16, ConstU64};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        StreamPayments: stream_payments::{Pallet, Call, Storage, Event<T>},
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
}

pub const STREAM_DEPOSIT: u64 = 100;
pub const MAX_STREAMS: u32 = 4;

frame_support::parameter_types! {
    pub const StreamDeposit: u64 = STREAM_DEPOSIT;
    pub const MaxStreams: u32 = MAX_STREAMS;
}

impl stream_payments::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type StreamDeposit = StreamDeposit;
    type MaxStreams = MaxStreams;
    type WeightInfo = stream_payments::weights::SubstrateWeight<Test>;
}

pub const A: u64 = 0;
pub const B: u64 = 1;
pub const INIT_BALANCE: u64 = 1_000_000;

#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    let genesis = pallet_balances::GenesisConfig::<Test> {
        balances: vec![(A, INIT_BALANCE), (B, INIT_BALANCE)],
    };
    genesis.assimilate_storage(&mut t).unwrap();
    t.into()
}
