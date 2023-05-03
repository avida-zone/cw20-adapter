use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub launchpad: String,
}

#[cw_serde]
pub struct ReceiveSubmsg {
    pub(crate) recipient: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    ///  Impl of Receiver CW-20 interface. Should be called by CW-20 contract only!! (never directly). Msg is ignored
    ///  Sender has already provided Proof on the rg-cw20 to do this adaptor action
    Receive { sender: String, amount: Uint128, msg: Binary },
    /// Called to redeem TF tokens. Will send rg-cw20 tokens to "recipient" address (or sender if not provided).
    /// Will use adaptor-transfer method on rg-cw20
    /// No Proof is needed, as the recipient will not be able to do futher transfer without proof
    RedeemAndTransfer { recipient: Option<String> },
    /// Updates stored metadata
    UpdateMetadata { addr: Addr },
    /// Registers a new denom on TF, called by launchpad automatically
    RegisterRG { addr: Addr },
}

#[cw_serde]
pub enum QueryMsg {
    /// Returns a fee required to register a new token-factory denom
    NewDenomFee {},
}
