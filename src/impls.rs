use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		// All storage in blockchain is Option<T>, so lets use default zero if not set
		let current_count = CountForKitties::<T>::get().unwrap_or(0);

		CountForKitties::<T>::set(Some(current_count + 1));

		// Maybe include new mint id here? (counter)
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
