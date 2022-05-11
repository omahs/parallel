
//! Autogenerated weights for `pallet_crowdloans`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("parallel-dev"), DB CACHE: 1024

// Executed Command:
// ./parallel
// benchmark
// pallet
// --chain=parallel-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_crowdloans
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=./runtime/parallel/src/weights/pallet_crowdloans.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_crowdloans`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_crowdloans::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans CTokensRegistry (r:1 w:1)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:1)
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Crowdloans NextTrieIndex (r:1 w:1)
	fn create_vault() -> Weight {
		(81_634_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	fn update_vault() -> Weight {
		(67_264_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Crowdloans IsVrf (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:3 w:3)
	// Storage: System Account (r:1 w:1)
	// Storage: XcmHelper XcmWeightFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Crowdloans XcmRequests (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	// Storage: unknown [0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291] (r:1 w:1)
	fn contribute() -> Weight {
		(300_938_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(19 as Weight))
			.saturating_add(T::DbWeight::get().writes(13 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	fn open() -> Weight {
		(63_532_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	fn close() -> Weight {
		(62_736_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans IsVrf (r:0 w:1)
	fn set_vrf() -> Weight {
		(33_796_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	fn reopen() -> Weight {
		(63_229_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	fn auction_succeeded() -> Weight {
		(62_922_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: XcmHelper XcmWeightFee (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Crowdloans XcmRequests (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn auction_failed() -> Weight {
		(207_457_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans CTokensRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: unknown [0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291] (r:1 w:1)
	fn claim() -> Weight {
		(128_452_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: unknown [0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291] (r:1 w:1)
	fn withdraw() -> Weight {
		(118_327_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans CTokensRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: Assets Asset (r:2 w:2)
	// Storage: Assets Account (r:2 w:2)
	fn redeem() -> Weight {
		(165_141_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: XcmHelper XcmWeightFee (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Crowdloans XcmRequests (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn slot_expired() -> Weight {
		(208_194_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:0)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: Crowdloans IsVrf (r:1 w:0)
	// Storage: XcmHelper XcmWeightFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Crowdloans XcmRequests (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	// Storage: unknown [0x] (r:1 w:0)
	// Storage: unknown [0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291] (r:2 w:2)
	fn migrate_pending() -> Weight {
		(296_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(17 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans XcmRequests (r:1 w:1)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: unknown [0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291] (r:2 w:2)
	fn notification_received() -> Weight {
		(176_468_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: unknown [0x] (r:3 w:0)
	fn refund() -> Weight {
		(137_976_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Crowdloans Vaults (r:1 w:1)
	// Storage: Crowdloans LeasesRegistry (r:1 w:1)
	// Storage: unknown [0x] (r:3 w:0)
	fn dissolve_vault() -> Weight {
		(145_279_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}