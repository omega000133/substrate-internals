use frame_support::{assert_noop, assert_ok, BoundedVec};

use super::*;
use crate::{mock::*, Error};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxProofLength> =
			vec![0, 1].try_into().unwrap();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxProofLength> =
			vec![0, 1].try_into().unwrap();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim),
			Error::<Test>::ProofAlreadyClaimed
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxProofLength> =
			vec![0, 1].try_into().unwrap();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim));
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxProofLength> =
			vec![0, 1].try_into().unwrap();

		assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim), Error::<Test>::NoSuchProof);
	})
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxProofLength> =
			vec![0, 1].try_into().unwrap();

		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim),
			Error::<Test>::NotProofOwner
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxProofLength> =
			vec![0, 1].try_into().unwrap();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), 2, claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxProofLength> =
			vec![0, 1].try_into().unwrap();

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), 2, claim),
			Error::<Test>::NoSuchProof
		);
	})
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim: BoundedVec<u8, <Test as Config>::MaxProofLength> =
			vec![0, 1].try_into().unwrap();

		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), 3, claim),
			Error::<Test>::NotProofOwner
		);
	})
}
