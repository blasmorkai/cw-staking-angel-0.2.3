use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{ to_binary, Addr, CosmosMsg,  StdResult, WasmMsg,  Coin};

pub use cw721::{OwnerOfResponse, TokensResponse, NftInfoResponse};
pub use crate::msg::QueryMsg;
pub use crate::msg::ExecuteMsg;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
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
            funds,
        }
        .into())
    }
}

