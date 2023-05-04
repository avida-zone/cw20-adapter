use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Addr, Coin, CosmosMsg, SubMsg,
};

use cw20_adapter::{error::ContractError, execute_register::handle_register_msg};
use injective_cosmwasm::{mock_dependencies, InjectiveMsg, InjectiveMsgWrapper, InjectiveRoute, WasmMockQuerier};

use common::{create_cw20_failing_info_query_handler, create_cw20_info_query_handler, create_denom_creation_fee_failing_handler};

use crate::common::{CONTRACT_ADDRESS, CW_20_ADDRESS, SENDER};
use avida_verifier::state::launchpad::{LaunchpadOptions, RG_CONTRACTS};
mod common;

// const CONTRACT_ADDRESS: &str = "inj1pvrwmjuusn9wh34j7y520g8gumuy9xtlt6xtzw";
// const CW_20_ADDRESS: &str = "inj1pjcw9hhx8kf462qtgu37p7l7shyqgpfr82r6em";
// const SENDER: &str = "inj1n0qvel0zfmsxu3q8q23xzjvuwfxn0ydlhgyh7h";

#[test]
fn it_handles_correct_register_msg_with_exact_funds() {
    let mut deps = mock_dependencies();
    deps.querier = WasmMockQuerier {
        smart_query_handler: create_cw20_info_query_handler(),
        ..Default::default()
    };

    RG_CONTRACTS
        .save(
            &mut deps.storage,
            Addr::unchecked(CW_20_ADDRESS),
            &LaunchpadOptions {
                launch_type: avida_verifier::state::launchpad::LaunchType::Transform("type".into()),
                originator: Addr::unchecked("mock"),
            },
        )
        .unwrap();
    let mut env = mock_env();
    env.contract.address = Addr::unchecked(CONTRACT_ADDRESS);
    let response = handle_register_msg(
        deps.as_mut(),
        env,
        mock_info(SENDER, &[Coin::new(10, "inj")]),
        Addr::unchecked(CW_20_ADDRESS),
    )
    .unwrap();

    assert_eq!(response.messages.len(), 1, "incorrect number of messages returned");

    if let SubMsg {
        msg: CosmosMsg::Custom(InjectiveMsgWrapper { route, msg_data }),
        ..
    } = response.messages.first().unwrap()
    {
        assert_eq!(route, &InjectiveRoute::Tokenfactory, "submessage had wrong route");
        if let InjectiveMsg::CreateDenom { sender, subdenom } = msg_data {
            assert_eq!(CONTRACT_ADDRESS, sender.as_str(), "incorrect sender in the create denom message");
            assert_eq!(CW_20_ADDRESS, subdenom.as_str(), "incorrect subdenom in the create denom message");
        } else {
            panic!("incorrect injective message found")
        }
    } else {
        panic!("incorrect submessage type found")
    }
}

#[test]
fn it_handles_correct_register_msg_with_extra_funds() {
    let mut deps = mock_dependencies();
    let mut env = mock_env();
    env.contract.address = Addr::unchecked(CONTRACT_ADDRESS);
    let response_err = handle_register_msg(
        deps.as_mut(),
        env,
        mock_info(SENDER, &[Coin::new(100, "inj"), Coin::new(20, "usdt")]),
        Addr::unchecked(CW_20_ADDRESS),
    )
    .unwrap_err();
    assert_eq!(response_err, ContractError::SuperfluousFundsProvided);
}

#[test]
fn it_returns_error_if_cannot_query_denom_creation_fee_register_msg() {
    let mut deps = mock_dependencies();
    deps.querier = WasmMockQuerier {
        token_factory_denom_creation_fee_handler: create_denom_creation_fee_failing_handler(),
        ..Default::default()
    };

    let response = handle_register_msg(
        deps.as_mut(),
        mock_env(),
        mock_info(SENDER, &[Coin::new(10, "inj")]),
        Addr::unchecked(CW_20_ADDRESS),
    )
    .unwrap_err();

    assert!(response.to_string().contains("custom error"), "incorrect error returned");
}

#[test]
fn it_returns_error_if_mismatched_denom_is_passed_register_msg() {
    let mut deps = mock_dependencies();
    let response = handle_register_msg(
        deps.as_mut(),
        mock_env(),
        mock_info(SENDER, &[Coin::new(10, "usdt")]),
        Addr::unchecked(CW_20_ADDRESS),
    )
    .unwrap_err();

    assert_eq!(response, ContractError::NotEnoughBalanceToPayDenomCreationFee, "incorrect error returned");
}

#[test]
fn it_returns_error_if_insufficient_coins_are_passed_register_msg() {
    let mut deps = mock_dependencies();

    let response = handle_register_msg(
        deps.as_mut(),
        mock_env(),
        mock_info(SENDER, &[Coin::new(9, "inj")]),
        Addr::unchecked(CW_20_ADDRESS),
    )
    .unwrap_err();

    assert_eq!(response, ContractError::NotEnoughBalanceToPayDenomCreationFee, "incorrect error returned");
}

#[test]
fn it_returns_error_if_no_coins_are_passed_register_msg() {
    let mut deps = mock_dependencies();
    let response = handle_register_msg(deps.as_mut(), mock_env(), mock_info(SENDER, &[]), Addr::unchecked(CW_20_ADDRESS)).unwrap_err();

    assert_eq!(response, ContractError::NotEnoughBalanceToPayDenomCreationFee, "incorrect error returned");
}

#[test]
fn it_returns_error_if_register_is_not_cw20_msg() {
    let mut deps = mock_dependencies();
    deps.querier = WasmMockQuerier {
        smart_query_handler: create_cw20_failing_info_query_handler(),
        ..Default::default()
    };

    let response = handle_register_msg(
        deps.as_mut(),
        mock_env(),
        mock_info(SENDER, &[Coin::new(10, "inj")]),
        Addr::unchecked(CW_20_ADDRESS),
    )
    .unwrap_err();

    assert_eq!(response, ContractError::NotCw20Address, "incorrect error returned");
}
