use substreams::key;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigDecimal, Deltas};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;

use crate::pb::contract::v1::Pools;

#[substreams::handlers::map]
fn graph_out(
    pools: Pools,
    tvl_deltas: Deltas<DeltaBigDecimal>,
) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();

    for pool in pools.pools {
        let token0 = pool.token0.unwrap();
        let token1 = pool.token1.unwrap();

        tables
            .create_row("Token", token0.address.clone())
            .set("name", token0.name)
            .set("symbol", token0.symbol)
            .set("decimals", token0.decimals as i32);
        tables
            .create_row("Token", token1.address.clone())
            .set("name", token1.name)
            .set("symbol", token1.symbol)
            .set("decimals", token1.decimals as i32);
        tables
            .create_row("Pool", pool.address)
            .set("token0", token0.address.clone())
            .set("token1", token1.address.clone())
            .set("createdAtTxHash", pool.created_at_tx_hash)
            .set(
                "createdAtBlockNumber",
                BigInt::from(pool.created_at_block_number),
            )
            .set(
                "createdAtTimestamp",
                BigInt::from(pool.created_at_timestamp),
            )
            .set("tvl", BigDecimal::zero());
    }

    for delta in tvl_deltas.deltas {
        let pool_address = key::segment_at(&delta.key, 1);
        tables
            .update_row("Pool", pool_address)
            .set("tvl", delta.new_value);
    }
    Ok(tables.to_entity_changes())
}
