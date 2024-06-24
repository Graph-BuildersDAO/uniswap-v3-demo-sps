use substreams::{scalar::BigInt, Hex};
use substreams_ethereum::rpc;

use crate::{abi::erc20_contract, pb::contract::v1::Token};

pub fn get_token(token_address: Vec<u8>) -> Token {
    // Batching RPC calls
    let batch = rpc::RpcBatch::new();

    let responses = batch
        .add(erc20_contract::functions::Name {}, token_address.clone())
        .add(erc20_contract::functions::Symbol {}, token_address.clone())
        .add(
            erc20_contract::functions::Decimals {},
            token_address.clone(),
        )
        .execute()
        .unwrap()
        .responses;

    let name = substreams_ethereum::rpc::RpcBatch::decode::<_, erc20_contract::functions::Name>(
        &responses[0],
    )
    .unwrap_or_else(|| {
        substreams::log::info!("failed to get name");
        "".to_string()
    });

    let symbol =
        substreams_ethereum::rpc::RpcBatch::decode::<_, erc20_contract::functions::Symbol>(
            &responses[1],
        )
        .unwrap_or_else(|| {
            substreams::log::debug!("failed to get symbol");
            "".to_string()
        });

    let decimals = substreams_ethereum::rpc::RpcBatch::decode::<
        _,
        erc20_contract::functions::Decimals,
    >(&responses[2])
    .unwrap_or_else(|| {
        substreams::log::debug!("failed to get decimals");
        BigInt::zero()
    });

    Token {
        address: Hex::encode(token_address),
        name,
        symbol,
        decimals: decimals.to_u64(),
    }
}
