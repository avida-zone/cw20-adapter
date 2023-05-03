use crate::common::{get_denom, is_contract_registered};
use crate::error::ContractError;
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, Uint128};
use injective_cosmwasm::{create_mint_tokens_msg, InjectiveMsgWrapper, InjectiveQueryWrapper};

pub fn handle_on_received_cw20_funds_msg(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    rg_sender: String,
    amount: Uint128,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    if !info.funds.is_empty() {
        return Err(ContractError::SuperfluousFundsProvided);
    }
    let rg_cw20_contract = info.sender;

    let response = Response::new();
    let master = env.contract.address;
    let denom = get_denom(&master, &rg_cw20_contract);

    // All RG should have been registered
    is_contract_registered(&deps, &rg_cw20_contract)?;

    let coins_to_mint = Coin::new(amount.u128(), denom);
    let mint_tf_tokens_message = create_mint_tokens_msg(master, coins_to_mint, rg_sender);

    Ok(response.add_message(mint_tf_tokens_message))
}
