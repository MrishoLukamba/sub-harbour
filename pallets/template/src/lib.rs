#![cfg_attr(not(feature = "std"), no_std)]
//There are 2 modules
//Crypto module for setting up offfchain workers crypto identification keys and implementation
//of AppCrypto trait which can be later be sued for configuring the pallet in the runime.
// Running this node , needs to setting a validator node manually using pre defined keys Alice and Bob
//
//I have just implement a simple ocw and a single dispatchable for testing .
//This Offchain worker runs every time a node which is Alice validating a block.
//
//Next is to set up http client and implement fecthing data and profe of receiving solution




use sp_runtime::offchain::KeyTypeId;


pub const KEY_TYPE:KeyTypeId = KeyTypeId(*b"offc");
pub use crypto::*;
pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto,sr25519},
		traits::Verify,
		MultiSignature, MultiSigner
	};

	app_crypto!(sr25519, KEY_TYPE);
	pub struct TestAuthId;

	//implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = Sr25519Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}


}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,

	};

	use frame_system::pallet_prelude::*;
	use frame_system::offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	};
	use sp_runtime::RuntimeDebug;
	use log;


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:CreateSignedTransaction<Call<Self>> + frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Call: From<Call<Self>>;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

	}

	type BlockNumberFor<T> = <T as frame_system::Config>::BlockNumber;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_value)]
	pub(super) type Value<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {


	}



	#[pallet::hooks]
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn offchain_worker(block_number: T::BlockNumber) {
			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			let results = signer.send_signed_transaction(|_account| {
				Call::expensive_func {num:21}
			});

			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("[{:?}]: submit success",acc.id),
					Err(e) =>log::error!("[{:?}]: poor, {:?}", acc.id,e)
				}
			}

		}

	}



	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn expensive_func(origin: OriginFor<T>, num: u32) ->DispatchResult {
			let _putter = ensure_signed(origin)?;
			let pre_result = num * 10_u32;
			let result = pre_result * 113_u32;
			<Value<T>>::put(result);
			Ok(())
		}


	}

  }


