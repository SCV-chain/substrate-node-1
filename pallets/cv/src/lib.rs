#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use utils::Content;

	pub struct Item<T::Config> {
		itemId: u32,
		userID: T::AccountId,
		created: WhoAndWhen<T>,
		org_date: Option(T::Moment),
		exp_date: Option(T::Moment),
		certificated: Certificate,
		metadata: Content,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn itemid)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type ItemId<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn item_by_id)]
	pub type ItemById<T> = StorageMap<_, twox_64_concat, TypeID, Item<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn items_by_accountid)]
	pub type ItemsByAccountId<T> = StorageMap<_, twox_64_concat, T::AccountId, Vec<Item<T>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RevokeSucceed(TypeID),
		CreateSucceed(TypeID),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000)]
		pub fn create_item(origin: OriginFor<T>, account_id: AccountId, metadata: String) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			let item_id = Self::item_id();
			let certificate =;
			let new_item: Item<T> = Item::new(item_id, account_id, who.clone(), metadata.clone());
			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::CreateSucceed(item_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_item(origin: OriginFor<T>, item_id: TypeID) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			let item_id = Self::item_id();
			let certificate =;
			let new_item: Item<T> = Item::new(item_id, account_id, who.clone(), metadata.clone());
			// Update storage.
			<ItemsByAccountId<T>>::get(who);

			// Emit an event.
			Self::deposit_event(Event::RevokeSucceed(item_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

	}
}
