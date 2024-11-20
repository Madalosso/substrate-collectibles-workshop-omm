// Tests for the Kitties Pallet.
//
// Normally this file would be split into two parts: `mock.rs` and `tests.rs`.
// The `mock.rs` file would contain all the setup code for our `TestRuntime`.
// Then `tests.rs` would only have the tests for our pallet.
// However, to minimize the project, these have been combined into this single file.
//
// Learn more about creating tests for Pallets:
// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html

// This flag tells rust to only run this file when running `cargo test`.
#![cfg(test)]

use crate as pallet_kitties;
use crate::*;
use frame::deps::sp_io;
use frame::runtime::prelude::*;
use frame::testing_prelude::*;
use frame::traits::fungible::*;

type Balance = u64;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// In our "test runtime", we represent a user `AccountId` with a `u64`.
// This is just a simplification so that we don't need to generate a bunch of proper cryptographic
// public keys when writing tests. It is just easier to say "user 1 transfers to user 2".
// We create the constants `ALICE` and `BOB` to make it clear when we are representing users below.
const ALICE: u64 = 1;
const BOB: u64 = 2;

const DEFAULT_KITTY: Kitty<TestRuntime> = Kitty { dna: [0u8; 32], owner: 0 };

// Our blockchain tests only need 3 Pallets:
// 1. System: Which is included with every FRAME runtime.
// 2. PalletBalances: Which is manages your blockchain's native currency. (i.e. DOT on Polkadot)
// 3. PalletKitties: The pallet you are building in this tutorial!
construct_runtime! {
	pub struct TestRuntime {
		System: frame_system,
		PalletBalances: pallet_balances,
		PalletKitties: pallet_kitties,
	}
}

// Normally `System` would have many more configurations, but you can see that we use some macro
// magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for TestRuntime {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

// Normally `pallet_balances` would have many more configurations, but you can see that we use some
// macro magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = Balance;
}

// This is the configuration of our Pallet! If you make changes to the pallet's `trait Config`, you
// will also need to update this configuration to represent that.
impl pallet_kitties::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
}

// We need to run most of our tests using this function: `new_test_ext().execute_with(|| { ... });`
// It simulates the blockchain database backend for our tests.
// If you forget to include this and try to access your Pallet storage, you will get an error like:
// "`get_version_1` called outside of an Externalities-provided environment."
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<TestRuntime>::default()
		.build_storage()
		.unwrap()
		.into()
}

#[test]
fn starting_template_is_sane() {
	new_test_ext().execute_with(|| {
		let event = Event::<TestRuntime>::Created { owner: ALICE };
		let _runtime_event: RuntimeEvent = event.into();
		let _call = Call::<TestRuntime>::create_kitty {};
		let result = PalletKitties::create_kitty(RuntimeOrigin::signed(BOB));
		assert_ok!(result);
	});
}

#[test]
fn system_and_balances_work() {
	// This test will just sanity check that we can access `System` and `PalletBalances`.
	new_test_ext().execute_with(|| {
		// We often need to set `System` to block 1 so that we can see events.
		System::set_block_number(1);
		// We often need to add some balance to a user to test features which needs tokens.
		assert_ok!(PalletBalances::mint_into(&ALICE, 100));
		assert_ok!(PalletBalances::mint_into(&BOB, 100));
	});
}

#[test]
fn create_kitty_checks_signed() {
	new_test_ext().execute_with(|| {
		// Function returns OK if create_kitty is called/signed by a valid AccountID
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));

		// functions fails if called from unsigned msg
		assert_noop!(PalletKitties::create_kitty(RuntimeOrigin::none()), DispatchError::BadOrigin);
	})
}

#[test]
fn create_kitty_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Execute call
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));

		// Assert last event is the Created one
		System::assert_last_event(Event::<TestRuntime>::Created { owner: 1 }.into());

		//Q: owner:1 -> 1 = ALICE?
	})
}

#[test]
fn count_for_kitties_created_correctly() {
	new_test_ext().execute_with(|| {
		assert_eq!(CountForKitties::<TestRuntime>::get(), u32::default());

		// A bit awkward... what is the difference between these 2?
		CountForKitties::<TestRuntime>::set(1337u32);
		assert_eq!(CountForKitties::<TestRuntime>::get(), 1337u32);
		CountForKitties::<TestRuntime>::put(1336u32);

		assert_ne!(CountForKitties::<TestRuntime>::get(), 1337u32);
		assert_eq!(CountForKitties::<TestRuntime>::get(), 1336u32);
	});
}

#[test]
fn mint_increment_counter() {
	new_test_ext().execute_with(|| {
		assert_eq!(CountForKitties::<TestRuntime>::get(), u32::default());

		// Check if necessary
		System::set_block_number(1);

		// Execute call
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));

		assert_eq!(CountForKitties::<TestRuntime>::get(), 1);
	})
}

#[test]
fn mint_error_on_overflow() {
	new_test_ext().execute_with(|| {
		CountForKitties::<TestRuntime>::put(u32::MAX);

		assert_noop!(
			PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)),
			Error::<TestRuntime>::TooManyKitties
		);
	});
}

// copied
#[test]
fn kitties_map_created_correctly() {
	new_test_ext().execute_with(|| {
		let zero_key = [0u8; 32];
		assert!(!Kitties::<TestRuntime>::contains_key(zero_key));
		Kitties::<TestRuntime>::insert(zero_key, DEFAULT_KITTY);
		assert!(Kitties::<TestRuntime>::contains_key(zero_key));
	})
}

//copied
#[test]
fn create_kitty_adds_to_map() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 1);
	})
}

#[test]
fn raise_error_on_duplicated() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(ALICE, [0u8; 32]));
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 1);

		assert_noop!(PalletKitties::mint(BOB, [0u8; 32]), Error::<TestRuntime>::DuplicatedKitty);
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 1);
	})
}

//copy
#[test]
fn kitty_struct_has_expected_traits() {
	new_test_ext().execute_with(|| {
		let kitty = DEFAULT_KITTY;
		let bytes = kitty.encode();
		let _decoded_kitty = Kitty::<TestRuntime>::decode(&mut &bytes[..]).unwrap();
		assert!(Kitty::<TestRuntime>::max_encoded_len() > 0);
		let _info = Kitty::<TestRuntime>::type_info();
	})
}

#[test]
fn mint_stores_owner_in_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(1337, [42u8; 32]));
		let kitty = Kitties::<TestRuntime>::get([42u8; 32]).unwrap();
		assert_eq!(kitty.owner, 1337);
		assert_eq!(kitty.dna, [42u8; 32]);
	})
}

#[test]
fn create_kitty_unique() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(BOB)));

		assert_eq!(CountForKitties::<TestRuntime>::get(), 2);
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 2);
	})
}

#[test]
fn kitties_owned_creation() {
	new_test_ext().execute_with(|| {
		// Initially users have no kitties owned.
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE).len(), 0);
		// Let's create two kitties.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		// Now they should have two kitties owned.
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE).len(), 2);
	})
}
