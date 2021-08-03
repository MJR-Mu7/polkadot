// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use crate::{account_id_junction, execute_order, execute_xcm, OverArchingCallOf, XcmCallOf};
use codec::Encode;
use frame_benchmarking::benchmarks_instance_pallet;
use frame_support::assert_ok;
use sp_std::{convert::TryInto, prelude::*, vec};
use xcm::{opaque::v0::MultiLocation, v0::Order};
use xcm_executor::{traits::TransactAsset, Assets};

// TODO: def. needs to be become a config, might also want to use bounded vec.
const MAX_ASSETS: u32 = 25;
/// The number of fungible assets in the holding.
const HOLDING_FUNGIBLES: u32 = 99;
const HOLDING_NON_FUNGIBLES: u32 = 99;

benchmarks_instance_pallet! {
	send_xcm {}: {}

	order_null {
		let order = Order::<XcmCallOf<T>>::Null;
		let origin = MultiLocation::X1(account_id_junction::<T>(1));
		let holding = Assets::default();
	}: {
		assert_ok!(execute_order::<T>(origin, holding, order));
	}
	order_deposit_asset {}: {} verify {}
	order_deposit_reserved_asset {}: {} verify {}
	order_exchange_asset {}: {} verify {}
	order_initiate_reserve_withdraw {}: {} verify {}
	order_initiate_teleport {}: {} verify {}
	order_query_holding {}: {} verify {}
	order_buy_execution {}: {} verify {}

	xcm_withdraw_asset {}: {} verify {}
	xcm_reserve_asset_deposit {}: {} verify {}
	xcm_teleport_asset {}: {} verify {}
	xcm_transfer_asset {}: {} verify {}
	xcm_transfer_reserved_asset {}: {} verify {}

	// xcm_query_response {}: {} verify {}
	// xcm_transact {}: {} verify {}
	// xcm_hrmp_channel_open_request {}: {} verify {}
	// xcm_hrmp_channel_accepted {}: {} verify {}
	// xcm_hrmp_channel_closing {}: {} verify {}
	// xcm_relayed_from {}: {} verify {}
}

#[cfg(test)]
mod benchmark_tests {
	use super::mock::{new_test_ext, Test};
	use super::*;

	#[test]
	fn order_deposit_asset_fungible() {
		new_test_ext().execute_with(|| {
			test_bench_by_name::<Test>(b"order_null").unwrap();
		})
	}
}