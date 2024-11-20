#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
mod tests;

use frame::prelude::*;
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: [u8; 32],
		pub owner: T::AccountId,
	}

	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery>;

	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 32], Value = Kitty<T>>;

	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> =
		StorageMap<Key = T::AccountId, Value = Vec<[u8; 32]>, QueryKind = ValueQuery>;

	// Using macro-magic
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
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
		NoKitty,
		DuplicatedKitty,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dna = Self::gen_dna();
			Self::mint(who, dna)?;
			Ok(())
		}
	}
}
