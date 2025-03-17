// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions from types declared in [`linera-sdk`] to types generated by [`wit-bindgen`].

use linera_base::{
    crypto::CryptoHash,
    data_types::{BlockHeight, Timestamp},
    http,
    identifiers::{ChainId, MultiAddress, Owner, UserApplicationId},
};

use crate::{
    contract::wit::base_runtime_api as base_contract_api,
    service::wit::base_runtime_api as base_service_api,
};

macro_rules! impl_to_wit {
    ($wit_base_api:ident) => {
        impl From<CryptoHash> for $wit_base_api::CryptoHash {
            fn from(hash_value: CryptoHash) -> Self {
                let parts = <[u64; 4]>::from(hash_value);

                $wit_base_api::CryptoHash {
                    part1: parts[0],
                    part2: parts[1],
                    part3: parts[2],
                    part4: parts[3],
                }
            }
        }

        impl From<Owner> for $wit_base_api::Owner {
            fn from(owner: Owner) -> Self {
                $wit_base_api::Owner {
                    inner0: owner.0.into(),
                }
            }
        }

        impl From<MultiAddress> for $wit_base_api::MultiAddress {
            fn from(account_owner: MultiAddress) -> Self {
                match account_owner {
                    MultiAddress::Address32(owner) => {
                        $wit_base_api::MultiAddress::Address32(owner.into())
                    }
                    MultiAddress::Chain => $wit_base_api::MultiAddress::Chain,
                }
            }
        }

        impl From<BlockHeight> for $wit_base_api::BlockHeight {
            fn from(block_height: BlockHeight) -> Self {
                $wit_base_api::BlockHeight {
                    inner0: block_height.0,
                }
            }
        }

        impl From<ChainId> for $wit_base_api::ChainId {
            fn from(chain_id: ChainId) -> Self {
                $wit_base_api::ChainId {
                    inner0: chain_id.0.into(),
                }
            }
        }

        impl From<UserApplicationId> for $wit_base_api::UserApplicationId {
            fn from(application_id: UserApplicationId) -> Self {
                $wit_base_api::UserApplicationId {
                    inner0: application_id.0.into(),
                }
            }
        }

        impl From<Timestamp> for $wit_base_api::Timestamp {
            fn from(timestamp: Timestamp) -> Self {
                Self {
                    inner0: timestamp.micros(),
                }
            }
        }

        impl From<http::Request> for $wit_base_api::HttpRequest {
            fn from(request: http::Request) -> Self {
                $wit_base_api::HttpRequest {
                    method: request.method.into(),
                    url: request.url,
                    headers: request
                        .headers
                        .into_iter()
                        .map(http::Header::into)
                        .collect(),
                    body: request.body,
                }
            }
        }

        impl From<http::Method> for $wit_base_api::HttpMethod {
            fn from(method: http::Method) -> Self {
                match method {
                    http::Method::Get => $wit_base_api::HttpMethod::Get,
                    http::Method::Post => $wit_base_api::HttpMethod::Post,
                    http::Method::Put => $wit_base_api::HttpMethod::Put,
                    http::Method::Delete => $wit_base_api::HttpMethod::Delete,
                    http::Method::Head => $wit_base_api::HttpMethod::Head,
                    http::Method::Options => $wit_base_api::HttpMethod::Options,
                    http::Method::Connect => $wit_base_api::HttpMethod::Connect,
                    http::Method::Patch => $wit_base_api::HttpMethod::Patch,
                    http::Method::Trace => $wit_base_api::HttpMethod::Trace,
                }
            }
        }

        impl From<http::Header> for $wit_base_api::HttpHeader {
            fn from(header: http::Header) -> Self {
                $wit_base_api::HttpHeader {
                    name: header.name,
                    value: header.value,
                }
            }
        }

        impl From<log::Level> for $wit_base_api::LogLevel {
            fn from(level: log::Level) -> Self {
                match level {
                    log::Level::Trace => $wit_base_api::LogLevel::Trace,
                    log::Level::Debug => $wit_base_api::LogLevel::Debug,
                    log::Level::Info => $wit_base_api::LogLevel::Info,
                    log::Level::Warn => $wit_base_api::LogLevel::Warn,
                    log::Level::Error => $wit_base_api::LogLevel::Error,
                }
            }
        }
    };
}

impl_to_wit!(base_service_api);
impl_to_wit!(base_contract_api);
