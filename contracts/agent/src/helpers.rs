use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg, WasmQuery, Coin
};

//use crate::msg::{ExecuteMsg, };

pub use cw721::{OwnerOfResponse, TokensResponse, NftInfoResponse};
// pub use cw721_base::QueryMsg;
// use cw721_base::ExecuteMsg;

pub use crate::msg::QueryMsg;
pub use crate::msg::ExecuteMsg;


// use crate::msg::ExecuteMsg;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AgentContract(pub Addr);

impl AgentContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T, funds:Vec<Coin>) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: funds,
        }
        .into())
    }

 
}
