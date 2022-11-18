use cosmwasm_std::{Addr, DepsMut, Env, Response};

use injective_cosmwasm::{create_set_token_metadata_msg, InjectiveMsgWrapper, InjectiveQueryWrapper};

use crate::common::{fetch_cw20_metadata, get_denom};
use crate::error::ContractError;
use crate::state::CW20_CONTRACTS;

pub fn handle_update_metadata(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    cw20_addr: Addr,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let contract_registered = CW20_CONTRACTS.contains(deps.storage, cw20_addr.as_str());
    if !contract_registered {
        return Err(ContractError::ContractNotRegistered);
    }
    let token_metadata = fetch_cw20_metadata(&deps, cw20_addr.as_str())?;

    let denom = get_denom(&env.contract.address, &cw20_addr);
    let set_metadata_message = create_set_token_metadata_msg(denom, token_metadata.name, token_metadata.symbol, token_metadata.decimals);

    Ok(Response::new().add_message(set_metadata_message))
}

#[cfg(test)]
mod tests {
    use crate::common::get_denom;
    use crate::common::test_utils::{create_cw20_info_query_handler, CONTRACT_ADDRESS, CW_20_ADDRESS};
    use crate::error::ContractError;
    use crate::execute_metadata::handle_update_metadata;
    use crate::state::CW20_CONTRACTS;
    use cosmwasm_std::testing::mock_env;
    use cosmwasm_std::{Addr, CosmosMsg, SubMsg};
    use injective_cosmwasm::{mock_dependencies, InjectiveMsg, InjectiveMsgWrapper, InjectiveRoute, WasmMockQuerier};

    #[test]
    fn it_updates_metadata() {
        let mut deps = mock_dependencies();
        deps.querier = WasmMockQuerier {
            smart_query_handler: create_cw20_info_query_handler(),
            ..Default::default()
        };
        let mut env = mock_env();
        env.contract.address = Addr::unchecked(CONTRACT_ADDRESS);
        CW20_CONTRACTS.insert(&mut deps.storage, CW_20_ADDRESS).unwrap();

        let response = handle_update_metadata(deps.as_mut(), env, Addr::unchecked(CW_20_ADDRESS)).unwrap();
        assert_eq!(response.messages.len(), 1, "incorrect number of messages returned");

        if let SubMsg {
            msg: CosmosMsg::Custom(InjectiveMsgWrapper { route, msg_data }),
            ..
        } = response.messages.get(0).unwrap()
        {
            assert_eq!(route, &InjectiveRoute::Tokenfactory, "submessage had wrong route");
            if let InjectiveMsg::SetTokenMetadata {
                denom,
                name,
                symbol,
                decimals,
            } = msg_data
            {
                assert_eq!(
                    get_denom(&Addr::unchecked(CONTRACT_ADDRESS), &Addr::unchecked(CW_20_ADDRESS)),
                    denom.as_str(),
                    "incorrect denom in set metadata message"
                );
                assert_eq!("SOL", symbol.as_str(), "incorrect symbol in set metadata message");
                assert_eq!("Solana", name.as_str(), "incorrect name in set metadata message");
                assert_eq!(6, *decimals, "incorrect decimals in set metadata message");
            } else {
                panic!("incorrect injective message found")
            }
        } else {
            panic!("incorrect submessage type found")
        }
    }

    #[test]
    fn it_tries_to_update_not_registered_contract() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let err_response = handle_update_metadata(deps.as_mut(), env, Addr::unchecked(CW_20_ADDRESS)).unwrap_err();
        assert_eq!(err_response, ContractError::ContractNotRegistered, "incorrect error");
    }
}
