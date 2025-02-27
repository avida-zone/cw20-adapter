use cosmwasm_std::{Addr, DepsMut, Env, Response};

use injective_cosmwasm::{create_set_token_metadata_msg, InjectiveMsgWrapper, InjectiveQueryWrapper};

use crate::common::{fetch_cw20_metadata, get_denom, is_contract_registered};
use crate::error::ContractError;

pub fn handle_update_metadata(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    cw20_addr: Addr,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    is_contract_registered(&deps, &cw20_addr)?;
    let token_metadata = fetch_cw20_metadata(&deps, cw20_addr.as_str())?;

    let denom = get_denom(&env.contract.address, &cw20_addr);
    let set_metadata_message = create_set_token_metadata_msg(denom, token_metadata.name, token_metadata.symbol, token_metadata.decimals);

    Ok(Response::new().add_message(set_metadata_message))
}
