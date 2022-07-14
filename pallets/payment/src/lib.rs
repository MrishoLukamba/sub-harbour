#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![feature(more_qualified_paths)]
//---This pallet mimic seller and user behaviour ---
//--------------------------------------------------
// The work flow is when the buy order is placed the offchain worker will submit -
// the location data transaction to the buy order tracker
// When the location matches the destination, the money reserved when buying the product is being released
// and sent to the seller.
//----------------------------------------------------
//------TASKS-----------------------------------------
//-- Configure Offchain worker to send signed transactions (ocw-file)
//--enable reserve currency trait when purchasing
//--call for updating location
//--logic for releaseing payment after the location data matches.
//--This pallet uses storage's for pallet-commerce

mod ocw;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::traits::tokens::currency::ReservableCurrency;
	use frame_support::{
		dispatch::PartialEq,
		traits::{Currency, ExistenceRequirement},
	};

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_commerce::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type Max: Get<u32>;
	}

	pub type AccountFor<T> = <T as frame_system::Config>::AccountId;
	pub type BalanceFor<T> = <<T as Config>::Currency as Currency<AccountFor<T>>>::Balance;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config>{
		PaymentLocked,
		PaymentReceived(T::AccountId,T::AccountId),
		LocationUpdated((u32,u32)),
		OrderPlaced,
	}

	#[pallet::storage]
	pub type OrderTracker<T:Config> = StorageValue<_, Order<T>>;

	#[pallet::storage]
	pub type Tracker<T:Config> = StorageValue<_, BoundedVec<(u32,u32),<T as Config>::Max>>;

	#[pallet::error]
	pub enum Error<T>{
		MaxReached,
		NotEnough,
	}

	#[derive(Encode, Decode,TypeInfo,MaxEncodedLen,PartialEq, Clone)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub enum Status{
		Done,
		Pending
	}

	#[derive(Encode, Decode,TypeInfo,MaxEncodedLen,PartialEq, Clone)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Order<T: Config> {
		pub Seller: T::AccountId,
		pub Buyer: T::AccountId,
		pub ItemId: [u8;16],
		pub Amount: BalanceFor<T>,
		pub Source: (u32,u32),
		pub Destination:(u32,u32),
		pub Time: T::BlockNumber,
		pub Completed:Status
	}


	#[pallet::hooks]
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T>{

	}

	#[pallet::call]
	impl<T:Config> Pallet<T>{
		#[pallet::weight(100)]
		pub fn update_location(origin:OriginFor<T>,location: (u32,u32))-> DispatchResult {
			let _ = ensure_signed(origin)?;

			Tracker::<T>::mutate(|val|{
				let val = val.as_mut().unwrap();
				val.try_push(location.clone())
			});

			Self::deposit_event(Event::LocationUpdated(location.clone()));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn purchase(origin: OriginFor<T>,
						item_id: [u8;16],
						source: (u32,u32),
						destination: (u32,u32),
						amount_paying: BalanceFor<T>,

		)-> DispatchResult{
			let buyer = ensure_signed(origin)?;
			let amount = <pallet_commerce::Items<T>>::get().unwrap().Price;

			<T as Config>::Currency::reserve(&buyer, amount_paying)?;

			let order_instance = Order::<T>{
				Seller: <pallet_commerce::Seller<T>>::get().unwrap(),
				Buyer: buyer,
				ItemId: item_id,
				Amount: amount_paying,
				Source: source,
				Destination:destination,
				Time:<frame_system::Pallet<T>>::block_number(),
				Completed:Status::Pending,
			};
			<OrderTracker<T>>::set(Some(order_instance));
			Self::deposit_event(Event::OrderPlaced);
			Ok(())
		}
	}

}
