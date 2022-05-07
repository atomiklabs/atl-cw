# Signed Decimal

## Why newtype?

CosmWasm offers two decimal types:
- [`Decimal`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/math/decimal.rs)
- [`Decimal256`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/math/decimal256.rs)

They both work on top of unsigned integers and are able to work as non-negative numeric values.

The Signed Decimal allows using a negative numeric value semantic.