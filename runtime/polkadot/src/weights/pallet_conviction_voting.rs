// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_conviction_voting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_conviction_voting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_conviction_voting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_conviction_voting::WeightInfo for WeightInfo<T> {
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(311), added: 2786, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `13517`
		//  Estimated: `42428`
		// Minimum execution time: 117_882_000 picoseconds.
		Weight::from_parts(120_344_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(311), added: 2786, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `14238`
		//  Estimated: `83866`
		// Minimum execution time: 184_958_000 picoseconds.
		Weight::from_parts(187_647_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn remove_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `13990`
		//  Estimated: `83866`
		// Minimum execution time: 155_331_000 picoseconds.
		Weight::from_parts(157_636_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn remove_other_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `13076`
		//  Estimated: `30706`
		// Minimum execution time: 62_283_000 picoseconds.
		Weight::from_parts(64_338_000, 0)
			.saturating_add(Weight::from_parts(0, 30706))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:512 w:512)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(311), added: 2786, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 512]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `29712 + r * (365 ±0)`
		//  Estimated: `83866 + r * (3411 ±0)`
		// Minimum execution time: 65_590_000 picoseconds.
		Weight::from_parts(1_649_043_960, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			// Standard Error: 141_963
			.saturating_add(Weight::from_parts(38_967_560, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 3411).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:512 w:512)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 512]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `29627 + r * (365 ±0)`
		//  Estimated: `83866 + r * (3411 ±0)`
		// Minimum execution time: 36_990_000 picoseconds.
		Weight::from_parts(1_610_127_966, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			// Standard Error: 142_409
			.saturating_add(Weight::from_parts(38_970_685, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 3411).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(311), added: 2786, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12290`
		//  Estimated: `30706`
		// Minimum execution time: 87_063_000 picoseconds.
		Weight::from_parts(89_213_000, 0)
			.saturating_add(Weight::from_parts(0, 30706))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
