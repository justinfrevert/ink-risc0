#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {
	use ink::prelude::{vec, vec::Vec};
	use risc0_zkvm::{serde::from_slice, SessionReceipt};
	use scale::Decode;
	// /// Defines the storage of your contract.
	// /// Add new fields to the below struct in order
	// /// to add new static storage fields to your contract.
	#[ink(storage)]
	pub struct Flipper {
		/// Stores a single `bool` value on the storage.
		value: bool,
	}

	impl Flipper {
		/// Constructor that initializes the `bool` value to the given `init_value`.
		#[ink(constructor)]
		pub fn new() -> Self {
			Self { value: false }
		}

		/// Constructor that initializes the `bool` value to `false`.
		///
		/// Constructors can delegate to other constructors.
		// #[ink(constructor)]
		// pub fn default() -> Self {
		// 	Self::new(Default::default())
		// }

		/// A message that can be called on instantiated contracts.
		/// This one flips the value of the stored `bool` from `true`
		/// to `false` and vice versa.
		#[ink(message)]
		// pub fn flip(&mut self, proof_bytes: Vec<u8>) {
		pub fn flip(&mut self, scale_decoded_receipt: Vec<u32>) {
			// Known image id for the current prover code
			let image_id: [u32; 8] = [
				3551133925, 2234817314, 2371648417, 2966256475, 711591402, 3149304623, 1597102258,
				534273939,
			];

			// if let Ok(scale_decoded_receipt) = &Vec::<u32>::decode(&mut &proof_bytes[..]) {
			let receipt: Result<SessionReceipt, _> = from_slice(&scale_decoded_receipt);

			if let Ok(receipt) = receipt {
				// Check verification of proof
				receipt.verify(image_id);
			}
			// }
		}
	}

	/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
	/// module and test functions are marked with a `#[test]` attribute.
	/// The below code is technically just normal Rust code.
	#[cfg(test)]
	mod tests {
		/// Imports all the definitions from the outer scope so we can use them here.
		use super::*;

		/// We test if the default constructor does its job.
		#[ink::test]
		fn default_works() {
			let flipper = Flipper::default();
			assert_eq!(flipper.get(), false);
		}

		/// We test a simple use case of our contract.
		#[ink::test]
		fn it_works() {
			let mut flipper = Flipper::new(false);
			assert_eq!(flipper.get(), false);

			let receipt = vec![];

			flipper.flip(receipt);
			assert_eq!(flipper.get(), true);
		}
	}

	/// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
	///
	/// When running these you need to make sure that you:
	/// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
	/// - Are running a Substrate node which contains `pallet-contracts` in the background
	#[cfg(all(test, feature = "e2e-tests"))]
	mod e2e_tests {
		/// Imports all the definitions from the outer scope so we can use them here.
		use super::*;

		/// A helper function used for calling contract messages.
		use ink_e2e::build_message;

		/// The End-to-End test `Result` type.
		type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

		/// We test that we can upload and instantiate the contract using its default constructor.
		#[ink_e2e::test]
		async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
			// Given
			let constructor = FlipperRef::default();

			// When
			let contract_account_id = client
				.instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
				.await
				.expect("instantiate failed")
				.account_id;

			// Then
			let get = build_message::<FlipperRef>(contract_account_id.clone())
				.call(|flipper| flipper.get());
			let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
			assert!(matches!(get_result.return_value(), false));

			Ok(())
		}

		/// We test that we can read and write a value from the on-chain contract contract.
		#[ink_e2e::test]
		async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
			// Given
			let constructor = FlipperRef::new(false);
			let contract_account_id = client
				.instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
				.await
				.expect("instantiate failed")
				.account_id;

			let get = build_message::<FlipperRef>(contract_account_id.clone())
				.call(|flipper| flipper.get());
			let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
			assert!(matches!(get_result.return_value(), false));

			// When
			let flip = build_message::<FlipperRef>(contract_account_id.clone())
				.call(|flipper| flipper.flip());
			let _flip_result =
				client.call(&ink_e2e::bob(), flip, 0, None).await.expect("flip failed");

			// Then
			let get = build_message::<FlipperRef>(contract_account_id.clone())
				.call(|flipper| flipper.get());
			let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
			assert!(matches!(get_result.return_value(), true));

			Ok(())
		}
	}
}
