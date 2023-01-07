#[cfg(test)]
mod tests {
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::helpers::{AgentContract};
    use cosmwasm_std::{coin, coins, to_binary, Addr, Coin, Empty, Uint128};
    use cw721::{OwnerOfResponse, NftInfoResponse};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};


    const NATIVE_DENOM: &str = "ujunox";
    const MANAGER1: &str = "juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4";
    const USER1: &str = "juno10c3slrqx3369mfsr9670au22zvq082jaejxx86";
    const USER2: &str = "juno10c3slrqx3369mfsr9670au22zvq082jaejxx87";
    const ADMIN: &str = MANAGER1;

    pub fn contract_agent() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    // pub fn contract_amm() -> Box<dyn Contract<Empty>> {
    //     let contract = ContractWrapper::new(
    //         crate::contract::execute,
    //         crate::contract::instantiate,
    //         crate::contract::query,
    //     )
    //     .with_reply(crate::contract::reply)
    //     .with_migrate(crate::contract::migrate);
    //     Box::new(contract)
    // }



    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER1),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(10000),
                    }],
                )
                .unwrap();
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER2),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(10000),
                    }],
                )
                .unwrap();
        })
    }

    fn store_code() -> (App, u64) {
        let mut app = mock_app();
        let code_id_nft = app.store_code(contract_agent());
        (app, code_id_nft)
    }

    // pub fn agent_angel_instantiate(app: &mut App, code_id: u64, agent: String, manager: String, treasury: String,) -> AgentContract {
    //     let msg = InstantiateMsg{ nft: todo!(), staking: todo!() };
    //     let contract = app
    //         .instantiate_contract(
    //             code_id,
    //             Addr::unchecked(MANAGER1),
    //             &msg,
    //             &[],
    //             "angel-staking",
    //             None,
    //         )
    //         .unwrap();
    //     AgentContract(contract)
    // }

 


    #[test]
    fn instantiate_mint_nft() {
  

    }

}