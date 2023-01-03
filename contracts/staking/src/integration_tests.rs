#[cfg(test)]
mod tests {
    use crate::{helpers::StakingContract, state::ValidatorInfo};
    use cosmwasm_std::{coin, coins, to_binary, Addr, Coin, Empty, Uint128, Decimal, Validator};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor, StakingInfo};
    use cw_utils::WEEK;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

    const NATIVE_DENOM: &str = "ujunox";
    const MANAGER1: &str = "juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4";
    const AGENT1: &str = "juno10c3slrqx3369mfsr9670au22zvq082jaejxx23";
    const TREASURY1: &str = "juno10c3slrqx3369mfsr9670au22zvq082jaejxx85";

    const NFT_ID1 :u128 = 1u128;
    const NFT_ID2 :u128 = 2u128;
    const NFT_ID3 :u128 = 3u128;

    const VALIDATOR1: &str = "AD4AA82AD0116B34848F152EF2CD86C5B061CE74";
    const VALIDATOR2: &str = "AD4AA82AD0116B34848F152EF2CD86C5B061CE75";
    const VALIDATOR3: &str = "AD4AA82AD0116B34848F152EF2CD86C5B061CE76";

    const USER1: &str = "juno10c3slrqx3369mfsr9670au22zvq082jaejxx86";
    const ADMIN: &str = MANAGER1;

    pub fn contract_staking() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    fn mock_app() -> App {
        AppBuilder::new().build(|router, api, storage| {
            let env = mock_env();
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(AGENT1),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1000),
                    }],
                )
                .unwrap();
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER1),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(2000),
                    }],
                )
                .unwrap();
        // Setup staking module for the correct mock data.                
        router
                .staking
                .setup(
                    storage,
                    StakingInfo {
                        bonded_denom: NATIVE_DENOM.to_string(),
                        unbonding_time: 1,
                        apr: Decimal::percent(10),
                    },
                )
                .unwrap();
        // Add mock validators
        router
            .staking
            .add_validator(
                api,
                storage,
                &env.block,
                Validator {
                    address: VALIDATOR1.to_string(),
                    commission: Decimal::zero(),
                    max_commission: Decimal::one(),
                    max_change_rate: Decimal::one(),
                },
            )
            .unwrap();
        router
            .staking
            .add_validator(
                api,
                storage,
                &env.block,
                Validator {
                    address: VALIDATOR2.to_string(),
                    commission: Decimal::zero(),
                    max_commission: Decimal::one(),
                    max_change_rate: Decimal::one(),
                },
            )
            .unwrap();
        router
            .staking
            .add_validator(
                api,
                storage,
                &env.block,
                Validator {
                    address: VALIDATOR3.to_string(),
                    commission: Decimal::zero(),
                    max_commission: Decimal::one(),
                    max_change_rate: Decimal::one(),
                },
            )
            .unwrap();
        })
    }

    fn store_code() -> (App, u64) {
        let mut app = mock_app();
        let code_id_staking = app.store_code(contract_staking());
        (app, code_id_staking)
    }

    pub fn staking_angel_instantiate(app: &mut App, code_id: u64, agent: String, manager: String, treasury: String,) -> StakingContract {
        let msg = InstantiateMsg{agent, manager, treasury};
        let contract = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(MANAGER1),
                &msg,
                &[],
                "angel-staking",
                None,
            )
            .unwrap();
        StakingContract(contract)
    }

    fn add_3_validators(
        app: &mut App,
        staking_contract: &StakingContract,
        sender: Addr,
        val1: String,
        val2: String,
        val3: String,
    ) {
        let msg = ExecuteMsg::AddValidator { address: val1.into(), bond_denom: NATIVE_DENOM.into(), unbonding_period: WEEK };
        app.execute_contract(sender.clone(), staking_contract.addr(), &msg, &[]).unwrap();
        let msg = ExecuteMsg::AddValidator { address: val2.into(), bond_denom: NATIVE_DENOM.into(), unbonding_period: WEEK };
        app.execute_contract(sender.clone(), staking_contract.addr(), &msg, &[]).unwrap();
        let msg = ExecuteMsg::AddValidator { address: val3.into(), bond_denom: NATIVE_DENOM.into(), unbonding_period: WEEK };
        app.execute_contract(sender.clone(), staking_contract.addr(), &msg, &[]).unwrap();
    }

    fn get_validator_info(app: &App, staking_contract: &StakingContract, val_address:String) -> ValidatorInfo {
        app.wrap()
            .query_wasm_smart(staking_contract.addr(), &QueryMsg::ValidatorInfo { address: val_address })
            .unwrap()
    }

//    #[test]
    fn add_three_validators() {
        let (mut app, code_id) = store_code();
        let staking_contract = staking_angel_instantiate(&mut app, code_id, AGENT1.into(), MANAGER1.into(), TREASURY1.into());
        // let msg = ExecuteMsg::AddValidator { address: VALIDATOR1.into(), bond_denom: NATIVE_DENOM.into(), unbonding_period: WEEK };


        // app.execute_contract(Addr::unchecked(MANAGER1), staking_contract.addr(), &msg, &[]).unwrap();
 
        // Validator 'AD4AA82AD0116B34848F152EF2CD86C5B061CE74' not in current validator set'
        // deps.querier.update_staking("ustake", &[sample_validator(VALIDATOR1),sample_validator(VALIDATOR2),sample_validator(VALIDATOR3)], &[]);
        // add_3_validators(&mut app, &staking_contract, Addr::unchecked(MANAGER1), VALIDATOR1.into(), VALIDATOR2.into(), VALIDATOR3.into());

        // let validator1_info = get_validator_info(&app, &staking_contract, VALIDATOR1.into());
        // assert_eq!(validator1_info, 
        //     ValidatorInfo{ 
        //         bond_denom: NATIVE_DENOM.into(), 
        //         unbonding_period: WEEK, 
        //         bonded: 0, 
        //         unbonding: 0, 
        //     }
        // )
    }

}