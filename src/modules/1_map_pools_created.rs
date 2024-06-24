use hex_literal::hex;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};

use crate::{
    abi::factory_contract::events::PoolCreated,
    pb::contract::v1::{Pool, Pools},
    rpc,
};

const UNISWAP_V3_FACTORY: [u8; 20] = hex!("1f98431c8ad98523631ae4a59f267346ea31f984");

#[substreams::handlers::map]
fn map_pools_created(blk: eth::Block) -> Result<Pools, substreams::errors::Error> {
    Ok(Pools {
        pools: blk
            .events::<PoolCreated>(&[&UNISWAP_V3_FACTORY])
            .map(|(event, log)| {
                let token0 = rpc::get_token(event.token0);
                let token1 = rpc::get_token(event.token1);

                return Pool {
                    address: Hex::encode(event.pool),
                    token0: Some(token0),
                    token1: Some(token1),
                    created_at_tx_hash: Hex(&log.receipt.transaction.hash).to_string(),
                    created_at_block_number: blk.number,
                    created_at_timestamp: blk.timestamp_seconds(),
                    log_ordinal: log.ordinal(),
                };
            })
            .collect(),
    })
}
