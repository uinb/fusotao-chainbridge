use super::*;
use crate as pallet_chainbridge_transfer;
use pallet_chainbridge as bridge;
use sp_runtime::{
    generic,
    traits::{AccountIdLookup, BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature,
};

pub use frame_support::{
    construct_runtime,
    pallet_prelude::GenesisBuild,
    parameter_types,
    traits::{
        ConstU128, ConstU32, Hooks, KeyOwnerProofSystem, OnFinalize, OnInitialize, Randomness,
        StorageInfo,
    },
    weights::{IdentityFee, Weight},
    PalletId, StorageValue,
};
use frame_system::EnsureRoot;
use sp_core::blake2_128;
use sp_runtime::{traits::AccountIdConversion, AccountId32};

pub(crate) type BlockNumber = u32;
pub type Signature = MultiSignature;
pub type Balance = u128;
pub type Index = u64;
pub type Hash = sp_core::H256;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub const MILLICENTS: Balance = 10_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const SS58Prefix: u16 = 42;
}
impl frame_system::Config for Test {
    type AccountData = pallet_balances::AccountData<Balance>;
    type AccountId = AccountId;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockHashCount = BlockHashCount;
    type BlockLength = ();
    type BlockNumber = BlockNumber;
    type BlockWeights = ();
    type Call = Call;
    type DbWeight = ();
    type Event = Event;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    type Index = Index;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type MaxConsumers = ConstU32<16>;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type Origin = Origin;
    type PalletInfo = PalletInfo;
    type SS58Prefix = SS58Prefix;
    type SystemWeightInfo = ();
    type Version = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Assets: pallet_assets::<Instance1>,
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        Bridge: pallet_chainbridge::{Pallet, Call, Storage, Event<T>},
        ChainBridgeTransfer: pallet_chainbridge_transfer::{Pallet, Call, Storage, Event<T>},
        Erc721: pallet_chainbridge_erc721::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const TestChainId: u8 = 5;
    pub const ProposalLifetime: u32 = 50;
}

impl bridge::Config for Test {
    type AdminOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type ChainId = TestChainId;
    type Event = Event;
    type Proposal = Call;
    type ProposalLifetime = ProposalLifetime;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 1 * DOLLARS;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

parameter_types! {
    pub const AssetDeposit: Balance = 100 * DOLLARS;
    pub const ApprovalDeposit: Balance = 1 * DOLLARS;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 10 * DOLLARS;
    pub const MetadataDepositPerByte: Balance = 1 * DOLLARS;
}

impl pallet_assets::Config<pallet_assets::Instance1> for Test {
    type ApprovalDeposit = ApprovalDeposit;
    type AssetAccountDeposit = ConstU128<DOLLARS>;
    type AssetDeposit = AssetDeposit;
    type AssetId = AssetId;
    type Balance = AssetBalance;
    type Currency = Balances;
    type Event = Event;
    type Extra = ();
    type ForceOrigin = EnsureRoot<AccountId>;
    type Freezer = ();
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type StringLimit = StringLimit;
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Test>;
}

parameter_types! {
    pub NativeTokenId: bridge::ResourceId = bridge::derive_resource_id(1, &blake2_128(b"DAV")); // native token id
    pub HashId: bridge::ResourceId = bridge::derive_resource_id(1, &blake2_128(b"hash"));
    pub Erc721Id: bridge::ResourceId = bridge::derive_resource_id(1, &blake2_128(b"NFT"));
    pub NativeTokenMaxValue : Balance = 1000_000_000_000_000_0000u128; // need to set correct value
}

impl pallet_chainbridge_erc721::Config for Test {
    type Event = Event;
    type Identifier = Erc721Id;
}
pub type AssetBalance = u128;
pub type AssetId = u32;

impl Config for Test {
    type AssetBalance = AssetBalance;
    type AssetId = AssetId;
    type AssetIdByName = ChainBridgeTransfer;
    type BridgeOrigin = bridge::EnsureBridge<Test>;
    type Call = Call;
    type Currency = Balances;
    type Erc721Id = Erc721Id;
    type Event = Event;
    type Fungibles = Assets;
    type HashId = HashId;
    type NativeTokenId = NativeTokenId;
    type NativeTokenMaxValue = NativeTokenMaxValue;
}

pub const RELAYER_A: AccountId32 = AccountId32::new([2u8; 32]);
pub const RELAYER_B: AccountId32 = AccountId32::new([3u8; 32]);
pub const RELAYER_C: AccountId32 = AccountId32::new([4u8; 32]);
pub const ENDOWED_BALANCE: Balance = 100 * DOLLARS;

pub fn new_test_ext() -> sp_io::TestExternalities {
    let bridge_id = PalletId(*b"oc/bridg").into_account();
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(bridge_id, ENDOWED_BALANCE), (RELAYER_A, ENDOWED_BALANCE)],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let r_id = bridge::derive_resource_id(0, b"BAR");

    pallet_chainbridge_transfer::GenesisConfig::<Test> {
        asset_id_by_resource_id: vec![(r_id, 999, "BAR".to_string())],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

fn last_event() -> Event {
    frame_system::Pallet::<Test>::events()
        .pop()
        .map(|e| e.event)
        .expect("Event expected")
}

pub fn expect_event<E: Into<Event>>(e: E) {
    assert_eq!(last_event(), e.into());
}

// Asserts that the event was emitted at some point.
pub fn event_exists<E: Into<Event>>(e: E) {
    let actual: Vec<Event> = frame_system::Pallet::<Test>::events()
        .iter()
        .map(|e| e.event.clone())
        .collect();
    let e: Event = e.into();
    let mut exists = false;
    for evt in actual {
        if evt == e {
            exists = true;
            break;
        }
    }
    assert!(exists);
}

// Checks events against the latest. A contiguous set of events must be provided. They must
// include the most recent event, but do not have to include every past event.
pub fn assert_events(mut expected: Vec<Event>) {
    let mut actual: Vec<Event> = frame_system::Pallet::<Test>::events()
        .iter()
        .map(|e| e.event.clone())
        .collect();

    expected.reverse();

    for evt in expected {
        let next = actual.pop().expect("event expected");
        assert_eq!(next, evt.into(), "Events don't match");
    }
}
