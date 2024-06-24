# Uniswap V3 Demo Substreams-powered Subgraph

This repository contains a Substreams-powered Subgraph (SPS) designed to track and analyze a basic subset of Uniswap V3 pools data. It serves as a practical reference for the "Builders DAO - Substreams Developer Basics Guide".

The repository demonstrates the use of general good practices for organizing and developing a Substreams project. It includes modules for extracting, transforming, and analyzing data from the Uniswap V3 protocol on the Ethereum blockchain.

The SPS makes use of the following Substreams features:
1. Event extraction and `Pool` creation from a known data-source.
2. Event extraction from data-source that are not known (e.g., events from `Pool`'s that are deployed by a Factory contract).
3. Use of RPC calls to fetch `Token` data.
4. Store modules to aggregate balances and TVL data.
5. `graph_out` module to emit `EntityChanges` that can be consumed by a Subgraph.

For a detailed guide on building and deploying Substreams packages, refer to the [Builders DAO - Substreams Developer Basics Guide](). // TODO add guide URL

## Project Structure

The repository is organized using general best practices for Substreams projects, ensuring modularity and maintainability.

- `src/`: Contains the Rust source code for the Substreams modules.
    - `src/modules/`: Contains all the Substreams modules, splitting these up allows for a cleaner codebase that is easier to navigate and understand.
    - `src/rpc/`: Contains all functionality related to RPC calls, in this case, a get_token function which gets a token protobuf using data from a batch RPC call.
    - `src/pb/`: Rust bindings generated from the protobuf definitions.
- `proto/`: Protobuf definitions for the data structures used in the project.
- `substreams.yaml`: Configuration file for the Substreams package.
- `subgraph.yaml`: Configuration file for the Subgraph.
- `schema.graphql`: Contains the entity definitions for the Subgraph.
- `Makefile`: Contains build and deployment commands for the project.

## Data Flow

```mermaid
graph TD;
  map_pools_created[map: map_pools_created];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_pools_created;
  store_pool_count[store: store_pool_count];
  map_pools_created --> store_pool_count;
  store_pools_created[store: store_pools_created];
  map_pools_created --> store_pools_created;
  map_pool_events[map: map_pool_events];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_pool_events;
  store_pools_created --> map_pool_events;
  store_pool_token_balances[store: store_pool_token_balances];
  map_pool_events --> store_pool_token_balances;
  store_pool_tvl[store: store_pool_tvl];
  store_pools_created --> store_pool_tvl;
  store_pool_token_balances --> store_pool_tvl;
  store_pool_token_balances -- deltas --> store_pool_tvl;
  chainlink_prices:chainlink_price_store --> store_pool_tvl;
  graph_out[map: graph_out];
  map_pools_created --> graph_out;
  store_pool_tvl -- deltas --> graph_out;
  chainlink_prices:store_confirmed_feeds[store: chainlink_prices:store_confirmed_feeds];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> chainlink_prices:store_confirmed_feeds;
  chainlink_prices:get_chainlink_answers[map: chainlink_prices:get_chainlink_answers];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> chainlink_prices:get_chainlink_answers;
  chainlink_prices:store_confirmed_feeds --> chainlink_prices:get_chainlink_answers;
  chainlink_prices:chainlink_price_store[store: chainlink_prices:chainlink_price_store];
  chainlink_prices:get_chainlink_answers --> chainlink_prices:chainlink_price_store;
  chainlink_prices:graph_out[map: chainlink_prices:graph_out];
  chainlink_prices:get_chainlink_answers --> chainlink_prices:graph_out;

```