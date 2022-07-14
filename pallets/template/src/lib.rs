#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
//This pallet is for testing the core functionality of less-trust payment purchasing

//-----------FLOW-----------------------------------------------
//configuring roles and items to be sold
//sellers can list items
// once buyer is purchasing he should fill up some paramters including id of the item ,-
//buyers location, and sellers id.
// and the value of destination will be the buyers location.
// a hook for updating sellers location.
//---------------------------------------------------------------
//........Tasks..................
//1. storage items for items
//      .struct for items {name, metadata{img(url),...}, hashed_id}
//----------------------------------------------------------------
//2. a struct for buy order
//      .buyer- AccountId,
//      .seller - AccountId,
//      .item id,
//      .initial-location -tuple of lat and long,
//      .final destination - tuple of lat and long,
//      . movement-tracker - array of tuple of lat and long
//      .completed - Option<done>
//---------------------------------------------------------------
//3. storage for seller < array of account Id>, a seller cannot be a buyer for now
//----------------------------------------------------------------
//4. storage for sellers order which should be updated when a buyer purchases something and-
// a seller completes an order
//----------------------------------------------------------------
//5. a storage for orders created by a buyer in which a seller can modify movement-tracker and
// update his location.
//----------------------------------------------------------------
//6. call for registry of sellers, 7. call for listing items.


pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use codec;
	use frame_support::{
		dispatch::PartialEq,
		traits::{Currency, ExistenceRequirement},
	};
	use sp_io::hashing::blake2_128;
	use sp_std::vec::Vec;
	use frame_support::storage::bounded_vec::BoundedVec;
	use frame_support::traits::tokens::currency::ReservableCurrency;


	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type Max: Get<u32>;
	}

	pub type AccountFor<T> = <T as frame_system::Config>::AccountId;
	pub type BalanceFor<T> = <<T as Config>::Currency as Currency<AccountFor<T>>>::Balance;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Items<T:Config> = StorageValue<_, Item<T>>;

	#[pallet::storage]
	pub type Seller<T:Config> = StorageValue<_, T::AccountId>;

//	#[pallet::storage]
//	pub type SellerOrders<T:Config> = StorageMap<_,Blake2_128Concat, T::AccountId, Order<T>, OptionQuery>;

//	#[pallet::storage]
//	pub type BuyerOrders<T:Config> = StorageMap<_,Blake2_128Concat, T::AccountId, Order<T>, OptionQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config>{
		ItemListed,
		SellerRegistered,
	}
	#[pallet::error]
	pub enum Error<T>{
		SellerNotBuyer,
		NotSeller,
	}


	#[derive(Encode, Decode,TypeInfo,MaxEncodedLen,PartialEq, Clone)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Item<T: Config> {
		pub Name: [u8;16],
		pub Price: BalanceFor<T>,
		pub Url: [u8;16],
		pub Seller:T::AccountId,
		pub id: [u8;16],
	}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(1000)]
		pub fn register_seller(origin: OriginFor<T>, seller: T::AccountId) -> DispatchResult{
			let _ = ensure_signed(origin)?;
			Seller::<T>::set(Some(seller));

			Self::deposit_event(Event::SellerRegistered);
			Ok(())
		}

		#[pallet::weight(1000)]
		pub fn list_item(origin: OriginFor<T>,
						 name: [u8;16],
						 url: [u8;16],
						 price: BalanceFor<T>,

		    ) -> DispatchResult{
			let seller = ensure_signed(origin)?;
			ensure!(Seller::<T>::get().unwrap()==seller,Error::<T>::NotSeller);

			let id = url.using_encoded(blake2_128);
			let ItemInstance = Item::<T>{
				Name: name,
				Price: price,
				Url: url,
				Seller: seller,
				id: id,
			};
			Items::<T>::set(Some(ItemInstance));
			Self::deposit_event(Event::ItemListed);
			Ok(())
		}
	}
}
