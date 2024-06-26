use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    // The address of the token factory module
    pub tokenfactory_module_address: String,
    // The denom of the token being tracked
    pub tracked_denom: String,
}

#[cw_serde]
pub enum SudoMsg {
    // Sudo endpoint called by chain before sending tokens
    // Errors returned by this endpoint will prevent the transaction from being sent
    BlockBeforeSend {
        // The address being sent from
        from: String,
        // The address being sent to
        to: String,
        // The amount and denom being sent
        amount: Coin,
    },
    // Sudo endpoint called by chain before sending tokens
    // Errors returned by this endpoint will NOT prevent the transaction from being sent
    TrackBeforeSend {
        // The address being sent from
        from: String,
        // The address being sent to
        to: String,
        // The amount and denom being sent
        amount: Coin,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Return the balance of the given address at the given timestamp.
    #[returns(Uint128)]
    BalanceAt {
        address: String,
        timestamp: Option<u64>,
    },
    /// Return the total supply at the given timestamp.
    #[returns(Uint128)]
    TotalSupplyAt { timestamp: Option<u64> },
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    /// Tracked denom
    pub tracked_denom: String,
    /// Token factory module address
    pub token_factory_module: String,
}
