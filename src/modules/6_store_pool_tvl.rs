use substreams::{
    key,
    scalar::{BigDecimal, BigInt},
    store::{
        DeltaBigInt, Deltas, StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetProto, StoreNew,
        StoreSet, StoreSetBigDecimal,
    },
};

use crate::pb::contract::v1::Pool;

#[substreams::handlers::store]
pub fn store_pool_tvl(
    pool_store: StoreGetProto<Pool>,
    balances_store: StoreGetBigInt,
    balances_deltas: Deltas<DeltaBigInt>,
    chainlink_prices: StoreGetBigDecimal,
    store: StoreSetBigDecimal,
) {
    // Iterate through balance deltas to identify changes in pool balances
    for delta in balances_deltas.deltas {
        let pool_address = key::segment_at(&delta.key, 1);
        let token_id = key::segment_at(&delta.key, 2);

        if let Some(pool) = pool_store.get_last(format!("pool:{}", &pool_address)) {
            let token0_balance: BigInt;
            let token1_balance: BigInt;

            if token_id == "0" {
                // Use `new_value` to avoid a store lookup
                token0_balance = delta.new_value;
                token1_balance = balances_store
                    .get_last(format!("pool_token_balance:{}:1", &pool_address))
                    .unwrap_or_else(|| BigInt::zero());
            } else {
                token0_balance = balances_store
                    .get_last(format!("pool_token_balance:{}:0", &pool_address))
                    .unwrap_or_else(|| BigInt::zero());
                // Use `new_value` to avoid a store lookup
                token1_balance = delta.new_value;
            }

            let token0 = pool.token0.unwrap();
            let token1 = pool.token1.unwrap();

            let token0_price = chainlink_prices
                .get_last(format!("price_by_symbol:{}:USD", &token0.symbol))
                .unwrap_or_else(|| BigDecimal::zero());
            let token1_price = chainlink_prices
                .get_last(format!("price_by_symbol:{}:USD", &token1.symbol))
                .unwrap_or_else(|| BigDecimal::zero());

            let token0_tvl = token0_balance.to_decimal(token0.decimals) * token0_price;
            let token1_tvl = token1_balance.to_decimal(token1.decimals) * token1_price;
            let tvl = token0_tvl + token1_tvl;

            store.set(delta.ordinal, format!("pool_tvl:{}", &pool_address), &tvl);
        }
    }
}
