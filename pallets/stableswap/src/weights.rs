// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::all)]

use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_stableswap
pub trait WeightInfo {
    fn get_delta() -> Weight;
    fn get_alternative_var() -> Weight;
    fn add_liquidity() -> Weight;
    fn remove_liquidity() -> Weight;
    fn create_pool() -> Weight;
}

/// Weights for stableswap using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn get_delta() -> Weight {
        10_000 as Weight
    }
    fn get_alternative_var() -> Weight {
        10_000 as Weight
    }
    fn add_liquidity() -> Weight {
        10_000 as Weight
    }
    fn remove_liquidity() -> Weight {
        10_000 as Weight
    }
    fn create_pool() -> Weight {
        10_000 as Weight
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn get_delta() -> Weight {
        10_000 as Weight
    }
    fn get_alternative_var() -> Weight {
        10_000 as Weight
    }
    fn add_liquidity() -> Weight {
        10_000 as Weight
    }
    fn remove_liquidity() -> Weight {
        10_000 as Weight
    }
    fn create_pool() -> Weight {
        10_000 as Weight
    }
}