use crate::common::query_denom_creation_fee;
use cosmwasm_std::{Coin, Deps, StdResult};
use injective_cosmwasm::InjectiveQueryWrapper;

pub fn new_denom_fee(deps: Deps<InjectiveQueryWrapper>) -> StdResult<Vec<Coin>> {
    query_denom_creation_fee(&deps.querier)
}
