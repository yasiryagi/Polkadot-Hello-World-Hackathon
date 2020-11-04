#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::traits::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// FIXME:
#[derive(Encode, Decode, Debug, Default, Clone, PartialEq, Eq)]
pub struct BurgerkingCode {
	css: Vec<u8>,
    variable: Vec<u8>,
    tweenmax: Vec<u8>,
}
// FIXME:


decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Something get(fn something): Option<u32>;
		BurgerCodeGetVar get(fn find_by_var): BurgerkingCode;
	}
}

// FIXME:
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		BurgerCodeSignTo(BurgerkingCode, AccountId),
		// ValueSet(u32, AccountId),
	}
);



// FIXME:
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_burgerking_code(origin, css: Vec<u8>, variable: Vec<u8>, tweenmax: Vec<u8>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			let burgermap = BurgerkingCode{
				css, variable, tweenmax,
				
			};
			BurgerCodeGetVar::put(burgermap.clone());
			Self::deposit_event(RawEvent::BurgerCodeSignTo(burgermap, who));

			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		StorageOverflow,
	}
}
