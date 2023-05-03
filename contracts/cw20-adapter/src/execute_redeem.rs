use avida_verifier::msg::rg_cw20::ExecuteMsg as RgExecMsg;
use cosmwasm_std::{to_binary, Addr, DepsMut, Env, MessageInfo, Response, WasmMsg};
use injective_cosmwasm::{create_burn_tokens_msg, InjectiveMsgWrapper, InjectiveQueryWrapper};

use crate::common::{is_contract_registered, AdapterCoin, AdapterDenom};
use crate::error::ContractError;

pub fn handle_redeem_msg(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let valid_recipient = recipient
        .map(|r| -> Result<Addr, _> { deps.api.addr_validate(&r) })
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    if info.funds.len() > 1 {
        return Err(ContractError::SuperfluousFundsProvided);
    }
    let tokens_to_exchange = info
        .funds
        .iter()
        .find_map(|c| -> Option<AdapterCoin> {
            match AdapterDenom::new(&c.denom) {
                Ok(denom) => Some(AdapterCoin { amount: c.amount, denom }),
                Err(_) => None,
            }
            // if denom_parser.is_match(&c.denom) {
            //     Some(c.clone())
            // } else {
            //     None
            // }
        })
        .ok_or(ContractError::NoRegisteredTokensProvided)?;

    let cw20_addr = tokens_to_exchange.denom.cw20_addr.clone();
    let burn_tf_tokens_message = create_burn_tokens_msg(env.contract.address, tokens_to_exchange.as_coin());

    // This is derived from what we added
    is_contract_registered(&deps, &Addr::unchecked(tokens_to_exchange.denom.cw20_addr))?;

    let adaptor_transfer_msg = WasmMsg::Execute {
        contract_addr: cw20_addr,
        msg: to_binary(&RgExecMsg::AdaptorTransfer {
            sender: info.sender,
            recipient: valid_recipient,
            amount: tokens_to_exchange.amount,
        })?,
        funds: vec![],
    };

    Ok(Response::new().add_message(adaptor_transfer_msg).add_message(burn_tf_tokens_message))
}
