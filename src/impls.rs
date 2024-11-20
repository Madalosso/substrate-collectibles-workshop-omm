use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
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

		Kitties::<T>::insert(dna, ());

		// Maybe include new mint id here? (counter)
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
