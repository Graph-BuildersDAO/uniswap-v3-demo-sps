use substreams::store::{StoreAdd, StoreAddInt64, StoreNew};

use crate::pb::contract::v1::Pools;

#[substreams::handlers::store]
pub fn store_pool_count(pools: Pools, store: StoreAddInt64) {
    for pool in pools.pools {
        store.add(pool.log_ordinal, "pools_count", 1)
    }
}
