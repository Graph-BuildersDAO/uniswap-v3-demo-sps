use std::str::FromStr;

use substreams::{
    scalar::BigInt,
    store::{StoreAdd, StoreAddBigInt, StoreNew},
};

use crate::pb::contract::v1::{pool_event::Type, PoolEvents};

#[substreams::handlers::store]
pub fn store_pool_token_balances(pool_events: PoolEvents, store: StoreAddBigInt) {
    for pool_event in pool_events.events {
        if let Some(event_type) = pool_event.r#type {
            match event_type {
                Type::MintEvent(mint) => {
                    store.add(
                        pool_event.log_ordinal,
                        format!("pool_token_balance:{}:0", &pool_event.pool_address),
                        BigInt::from_str(&mint.amount0).unwrap_or_else(|_| BigInt::zero()),
                    );
                    store.add(
                        pool_event.log_ordinal,
                        format!("pool_token_balance:{}:1", &pool_event.pool_address),
                        BigInt::from_str(&mint.amount1).unwrap_or_else(|_| BigInt::zero()),
                    )
                }
                Type::BurnEvent(burn) => {
                    store.add(
                        pool_event.log_ordinal,
                        format!("pool_token_balance:{}:0", &pool_event.pool_address),
                        BigInt::from_str(&burn.amount0)
                            .unwrap_or_else(|_| BigInt::zero())
                            .neg(),
                    );
                    store.add(
                        pool_event.log_ordinal,
                        format!("pool_token_balance:{}:1", &pool_event.pool_address),
                        BigInt::from_str(&burn.amount1)
                            .unwrap_or_else(|_| BigInt::zero())
                            .neg(),
                    )
                }
                Type::SwapEvent(swap) => {
                    store.add(
                        pool_event.log_ordinal,
                        format!("pool_token_balance:{}:0", &pool_event.pool_address),
                        BigInt::from_str(&swap.amount0).unwrap_or_else(|_| BigInt::zero()),
                    );
                    store.add(
                        pool_event.log_ordinal,
                        format!("pool_token_balance:{}:1", &pool_event.pool_address),
                        BigInt::from_str(&swap.amount1).unwrap_or_else(|_| BigInt::zero()),
                    )
                }
            }
        }
    }
}
