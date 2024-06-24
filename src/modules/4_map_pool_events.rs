use substreams::{
    store::{StoreGet, StoreGetProto},
    Hex,
};
use substreams_ethereum::{pb::eth::v2 as eth, Event};

use crate::{
    abi::pool_contract::events::{Burn, Mint, Swap},
    pb::contract::v1::{pool_event, Pool, PoolEvent, PoolEvents},
};

#[substreams::handlers::map]
pub fn map_pool_events(
    blk: eth::Block,
    pool_store: StoreGetProto<Pool>,
) -> Result<PoolEvents, substreams::errors::Error> {
    let mut pool_events = PoolEvents::default();

    for trx in blk.transactions() {
        for (log, _) in trx.logs_with_calls() {
            let pool_address = Hex::encode(&log.address);
            if let Some(pool) = pool_store.get_last(format!("pool:{}", &pool_address)) {
                substreams::log::debug!("Found pool: {}", &pool_address);
                if let Some(swap) = Swap::match_and_decode(&log) {
                    pool_events.events.push(PoolEvent {
                        r#type: Some(pool_event::Type::SwapEvent(pool_event::SwapEvent {
                            sender: Hex::encode(&swap.sender),
                            recipient: Hex::encode(&swap.recipient),
                            amount0: swap.amount0.to_string(),
                            amount1: swap.amount1.to_string(),
                            tick: swap.tick.to_string(),
                        })),
                        pool_address: pool.address.clone(),
                        tx_hash: Hex::encode(&trx.hash),
                        block_number: blk.number,
                        timestamp: blk.timestamp_seconds(),
                        log_ordinal: log.ordinal,
                    })
                }
                if let Some(mint) = Mint::match_and_decode(&log) {
                    pool_events.events.push(PoolEvent {
                        r#type: Some(pool_event::Type::MintEvent(pool_event::MintEvent {
                            sender: Hex::encode(&mint.sender),
                            owner: Hex::encode(&mint.owner),
                            amount0: mint.amount0.to_string(),
                            amount1: mint.amount1.to_string(),
                            amount: mint.amount.to_string(),
                            tick_lower: mint.tick_lower.to_string(),
                            tick_upper: mint.tick_upper.to_string(),
                        })),
                        pool_address: pool.address.clone(),
                        tx_hash: Hex::encode(&trx.hash),
                        block_number: blk.number,
                        timestamp: blk.timestamp_seconds(),
                        log_ordinal: log.ordinal,
                    })
                }
                if let Some(burn) = Burn::match_and_decode(&log) {
                    pool_events.events.push(PoolEvent {
                        r#type: Some(pool_event::Type::BurnEvent(pool_event::BurnEvent {
                            tick_lower: burn.tick_lower.to_string(),
                            tick_upper: burn.tick_upper.to_string(),
                            amount: burn.amount.to_string(),
                            amount0: burn.amount0.to_string(),
                            amount1: burn.amount1.to_string(),
                        })),
                        pool_address: pool.address.clone(),
                        tx_hash: Hex::encode(&trx.hash),
                        block_number: blk.number,
                        timestamp: blk.timestamp_seconds(),
                        log_ordinal: log.ordinal,
                    })
                }
            }
        }
    }
    Ok(pool_events)
}
