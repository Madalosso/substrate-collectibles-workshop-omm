#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
mod tests;

use frame::prelude::*;
use frame::traits::fungible::{Inspect, Mutate};
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Access the balances pallet through the associated type `NativeBalance`.
		/// The `NativeBalance` type must implement `Inspect` and `Mutate`.
		/// Both of these traits are generic over the `AccountId` type.
		type NativeBalance: Inspect<Self::AccountId> + Mutate<Self::AccountId>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: [u8; 32],
		pub owner: T::AccountId,
		pub price: Option<BalanceOf<T>>,
	}

	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery>;

	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 32], Value = Kitty<T>>;

	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		Key = T::AccountId,
		Value = BoundedVec<[u8; 32], ConstU32<100>>,
		QueryKind = ValueQuery,
	>;

	// Using macro-magic
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32] },
	}

	// Without macro-magic (Revisit this later)
	// #[pallet::event]
	// pub(super) fn deposit_event(event: Event<T>) {
	// 	let event = <<T as Config>::RuntimeEvent as From<Event<T>>>::from(event);
	// 	let event = <<T as Config>::RuntimeEvent as Into<
	// 		<T as frame_system::Config>::RuntimeEvent,
	// 	>>::into(event);
	// 	<frame_system::Pallet<T>>::deposit_event(event)
	// }
	// pub enum Event<T: Config> {
	// 	Created { owner: T::AccountId },
	// }

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicatedKitty,
		NoKitty,
		TooManyOwned,
		NotOwner,
		TransferToSelf,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dna = Self::gen_dna();
			Self::mint(who, dna)?;
			Ok(())
		}

		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: [u8; 32],
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			Self::do_transfer(from, to, kitty_id)?;
			Ok(())
		}
	}
}
