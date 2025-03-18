// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions from types generated by [`wit-bindgen`] to types declared in [`linera-sdk`].

use linera_base::{
    crypto::CryptoHash,
    data_types::{Amount, BlockHeight, TimeDelta, Timestamp},
    http,
    identifiers::{ChainId, MultiAddress},
    ownership::{ChainOwnership, TimeoutConfig},
};

use crate::{
    contract::wit::base_runtime_api as base_contract_api,
    service::wit::base_runtime_api as base_service_api,
};

macro_rules! impl_from_wit {
    ($wit_base_api:ident) => {
        impl From<$wit_base_api::CryptoHash> for CryptoHash {
            fn from(hash_value: $wit_base_api::CryptoHash) -> Self {
                CryptoHash::from([
                    hash_value.part1,
                    hash_value.part2,
                    hash_value.part3,
                    hash_value.part4,
                ])
            }
        }

        impl From<$wit_base_api::MultiAddress> for MultiAddress {
            fn from(account_owner: $wit_base_api::MultiAddress) -> Self {
                match account_owner {
                    $wit_base_api::MultiAddress::Address32(owner) => {
                        MultiAddress::Address32(owner.into())
                    }
                }
            }
        }

        impl From<$wit_base_api::Amount> for Amount {
            fn from(balance: $wit_base_api::Amount) -> Self {
                let (lower_half, upper_half) = balance.inner0;
                let value = ((upper_half as u128) << 64) | (lower_half as u128);
                Amount::from_attos(value)
            }
        }

        impl From<$wit_base_api::BlockHeight> for BlockHeight {
            fn from(block_height: $wit_base_api::BlockHeight) -> Self {
                BlockHeight(block_height.inner0)
            }
        }

        impl From<$wit_base_api::ChainId> for ChainId {
            fn from(chain_id: $wit_base_api::ChainId) -> Self {
                ChainId(chain_id.inner0.into())
            }
        }

        impl From<$wit_base_api::Timestamp> for Timestamp {
            fn from(timestamp: $wit_base_api::Timestamp) -> Self {
                Timestamp::from(timestamp.inner0)
            }
        }

        impl From<$wit_base_api::TimeDelta> for TimeDelta {
            fn from(guest: $wit_base_api::TimeDelta) -> Self {
                TimeDelta::from_micros(guest.inner0)
            }
        }

        impl From<$wit_base_api::TimeoutConfig> for TimeoutConfig {
            fn from(guest: $wit_base_api::TimeoutConfig) -> TimeoutConfig {
                let $wit_base_api::TimeoutConfig {
                    fast_round_duration,
                    base_timeout,
                    timeout_increment,
                    fallback_duration,
                } = guest;
                TimeoutConfig {
                    fast_round_duration: fast_round_duration.map(TimeDelta::from),
                    base_timeout: base_timeout.into(),
                    timeout_increment: timeout_increment.into(),
                    fallback_duration: fallback_duration.into(),
                }
            }
        }

        impl From<$wit_base_api::ChainOwnership> for ChainOwnership {
            fn from(guest: $wit_base_api::ChainOwnership) -> ChainOwnership {
                let $wit_base_api::ChainOwnership {
                    super_owners,
                    owners,
                    multi_leader_rounds,
                    open_multi_leader_rounds,
                    timeout_config,
                } = guest;
                ChainOwnership {
                    super_owners: super_owners.into_iter().map(Into::into).collect(),
                    owners: owners
                        .into_iter()
                        .map(|(owner, weight)| (owner.into(), weight))
                        .collect(),
                    multi_leader_rounds,
                    open_multi_leader_rounds,
                    timeout_config: timeout_config.into(),
                }
            }
        }

        impl From<$wit_base_api::HttpResponse> for http::Response {
            fn from(response: $wit_base_api::HttpResponse) -> http::Response {
                http::Response {
                    status: response.status,
                    headers: response
                        .headers
                        .into_iter()
                        .map(http::Header::from)
                        .collect(),
                    body: response.body,
                }
            }
        }

        impl From<$wit_base_api::HttpHeader> for http::Header {
            fn from(header: $wit_base_api::HttpHeader) -> http::Header {
                http::Header::new(header.name, header.value)
            }
        }
    };
}

impl_from_wit!(base_service_api);
impl_from_wit!(base_contract_api);
