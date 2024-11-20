use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		// All storage in blockchain is Option<T>, so lets use default zero if not set
		let current_count = CountForKitties::<T>::get();

		// Error didn't have to include .into() due to the "?" sign.
		let updated_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		CountForKitties::<T>::set(updated_count);

		// Maybe include new mint id here? (counter)
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
