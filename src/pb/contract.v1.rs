// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub token0: ::core::option::Option<Token>,
    #[prost(message, optional, tag="3")]
    pub token1: ::core::option::Option<Token>,
    #[prost(string, tag="4")]
    pub created_at_tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub created_at_block_number: u64,
    #[prost(uint64, tag="6")]
    pub created_at_timestamp: u64,
    #[prost(uint64, tag="7")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<PoolEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeAmountEnabledCall {
    #[prost(string, tag="1")]
    pub fee: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tick_spacing: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolEvent {
    #[prost(string, tag="4")]
    pub pool_address: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub block_number: u64,
    #[prost(uint64, tag="7")]
    pub timestamp: u64,
    #[prost(uint64, tag="8")]
    pub log_ordinal: u64,
    #[prost(oneof="pool_event::Type", tags="1, 2, 3")]
    pub r#type: ::core::option::Option<pool_event::Type>,
}
/// Nested message and enum types in `PoolEvent`.
pub mod pool_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SwapEvent {
        #[prost(string, tag="1")]
        pub sender: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub amount0: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub amount1: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub tick: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MintEvent {
        #[prost(string, tag="1")]
        pub sender: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub owner: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub amount0: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub amount1: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag="6")]
        pub tick_lower: ::prost::alloc::string::String,
        #[prost(string, tag="7")]
        pub tick_upper: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BurnEvent {
        #[prost(string, tag="1")]
        pub tick_lower: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub tick_upper: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub amount0: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub amount1: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        SwapEvent(SwapEvent),
        #[prost(message, tag="2")]
        MintEvent(MintEvent),
        #[prost(message, tag="3")]
        BurnEvent(BurnEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedCalls {
    #[prost(message, repeated, tag="1")]
    pub failed_calls: ::prost::alloc::vec::Vec<FailedCall>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedCall {
    #[prost(string, tag="1")]
    pub call_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub caller: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reason: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub gas_consumed: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="3")]
    pub factory_pool_createds: ::prost::alloc::vec::Vec<FactoryPoolCreated>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryPoolCreated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub token0: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token1: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub fee: u64,
    #[prost(int64, tag="8")]
    pub tick_spacing: i64,
    #[prost(bytes="vec", tag="9")]
    pub pool: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
