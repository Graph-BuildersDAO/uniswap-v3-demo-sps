use substreams::store::{StoreNew, StoreSet, StoreSetProto};

use crate::pb::contract::v1::{Pool, Pools};

#[substreams::handlers::store]
fn store_pools_created(pools: Pools, store: StoreSetProto<Pool>) {
    for pool in pools.pools {
        store.set(pool.log_ordinal, format!("pool:{}", &pool.address), &pool)
    }
}
