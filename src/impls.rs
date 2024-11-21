use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::traits::Hash;

impl<T: Config> Pallet<T> {
	pub fn gen_dna() -> [u8; 32] {
		let unique_payload = (
			frame_system::Pallet::<T>::parent_hash(),
			frame_system::Pallet::<T>::block_number(),
			frame_system::Pallet::<T>::extrinsic_index(),
			CountForKitties::<T>::get(),
		);
		let hash: [u8; 32] = BlakeTwo256::hash_of(&unique_payload).into();
		return hash;
	}

	pub fn do_transfer(from: T::AccountId, to: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		// Not transferring to self
		ensure!(!from.eq(&to), Error::<T>::TransferToSelf);

		// Kitty exists
		// ensure!(Kitties::<T>::contains_key(dna), Error::<T>::NoKitty);
		let mut kitty = match Kitties::<T>::get(dna) {
			Some(kitty) => kitty,
			None => return Err(Error::<T>::NoKitty.into()),
		};
		// caller/from owner
		ensure!(kitty.owner.eq(&from), Error::<T>::NotOwner);

		// Add kitty to owned map
		let mut to_owned = KittiesOwned::<T>::get(to.clone());
		to_owned.try_push(dna).map_err(|_| Error::<T>::TooManyOwned)?;

		// Valid alternative? If so, maybe this removes the necessity of
		// KittiesOwned::<T>::insert(to, to_owned);
		// later?
		// KittiesOwned::<T>::try_append(&to, dna).map_err(|_| Error::<T>::TooManyOwned)?;

		// Remove kitty from from_owned map
		// let mut from_owned = KittiesOwned::<T>::try_get(from)?;
		let mut from_owned = KittiesOwned::<T>::get(from.clone());

		let index =
			from_owned
				.iter()
				.enumerate()
				.find_map(|(index, &item)| if item.eq(&dna) { Some(index) } else { None });

		let index = match index {
			Some(index) => index,
			None => return Err(Error::<T>::NotOwner.into()),
		};
		from_owned.swap_remove(index);

		kitty.owner = to.clone();
		Kitties::<T>::insert(dna, kitty);
		KittiesOwned::<T>::insert(to.clone(), to_owned);
		KittiesOwned::<T>::insert(from.clone(), from_owned);

		Self::deposit_event(Event::<T>::Transferred { from, to, kitty_id: dna });
		Ok(())
	}

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let kitty = Kitty { dna, owner: owner.clone() };

		// Ensure dna not present already
		// match Kitties::<T>::contains_key(dna) {
		// 	true => return Err(Error::<T>::DuplicatedKitty.into()),
		// 	false => {},
		// }
		// Macro instead:
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicatedKitty);

		// All storage in blockchain is Option<T>, so lets use default zero if not set
		let current_count = CountForKitties::<T>::get();

		// Error didn't have to include .into() due to the "?" sign.
		let updated_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		CountForKitties::<T>::set(updated_count);
		Kitties::<T>::insert(dna, kitty);
		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyOwned)?;

		// Maybe include new mint id here? (counter)
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
