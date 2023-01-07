#[cfg(test)]
mod tests {
    use crate::{helpers::StakingContract, state::ValidatorInfo};
    use cosmwasm_std::{coin, coins, to_binary, Addr, Coin, Empty, Uint128, Decimal, Validator, FullDelegation, StdResult};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor, StakingInfo};
    use cw_utils::WEEK;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

    const NATIVE_DENOM: &str = "ujunox";
    const MANAGER1: &str = "juno148v3g2dpjeq6hwnlagmvq8pnqe5r9wjcrvel8u";
    const AGENT1: &str = "juno15urq2dtp9qce4fyc85m6upwm9xul30492fasy3";
    const TREASURY1: &str = "juno196ax4vc0lwpxndu9dyhvca7jhxp70rmcl99tyh";

    const NFT_ID1 :u128 = 1u128;
    const NFT_ID2 :u128 = 2u128;
    const NFT_ID3 :u128 = 3u128;

    // const VALIDATOR1: &str = "AD4AA82AD0116B34848F152EF2CD86C5B061CE74";
    const VALIDATOR1: &str = "validator1";
    const VALIDATOR2: &str = "validator2";
    const VALIDATOR3: &str = "validator3";

    const USER1: &str = "juno10c3slrqx3369mfsr9670au22zvq082jaejxx86";
    // const ADMIN: &str = MANAGER1;
    const ADMIN: &str = "ADMIN";

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
                        amount: Uint128::new(5000),
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

    fn get_bonded_by_nft(app: &App, staking_contract: &StakingContract, nft_id:String) -> Uint128 {
        app.wrap()
            .query_wasm_smart(staking_contract.addr(), &QueryMsg::BondedByNFT { nft_id })
            .unwrap()
    }

    fn get_bonded_on_validator(app: &App, staking_contract: &StakingContract, validator:&str) -> StdResult<Uint128> {
        let delegation = app.wrap()
            .query_wasm_smart(staking_contract.addr(), &QueryMsg::BondedOnValidator { address: validator.to_string() })
            .unwrap();
        Ok(delegation)
    }

    pub fn query_module_delegation(
        app: &App,
        delegator: &str,
        validator: &str,
    ) -> Option<FullDelegation> {
        app.wrap().query_delegation(delegator, validator).unwrap()
    }
    
    pub fn query_rewards(app: &App, delegator: &str, validator: &str) -> Option<Uint128> {
        let rewards = query_module_delegation(app, delegator, validator)
            .unwrap()
            .accumulated_rewards;
    
        if rewards.is_empty() {
            None
        } else {
            Some(rewards[0].amount)
        }
    }

    fn get_balance(app: &App, user: String, denom: String) -> Coin {
        app.wrap().query_balance(user, denom).unwrap()
    }

    #[test]
    fn bond_unbond_claim() {
        let (mut app, code_id) = store_code();
        let staking_contract = staking_angel_instantiate(&mut app, code_id, AGENT1.into(), MANAGER1.into(), TREASURY1.into());
        // Add Three validators
        add_3_validators(&mut app, &staking_contract, Addr::unchecked(MANAGER1), VALIDATOR1.into(), VALIDATOR2.into(), VALIDATOR3.into());
        let validator1_info = get_validator_info(&app, &staking_contract, VALIDATOR1.into());
        assert_eq!(validator1_info, 
            ValidatorInfo{ 
                bond_denom: NATIVE_DENOM.into(), 
                unbonding_period: WEEK, 
                bonded: 0, 
                unbonding: 0, 
            }
        );
        let balance = get_balance(&app, AGENT1.to_string(), NATIVE_DENOM.to_string());
        println!(">>>>>>>>>>>>> AGENT1 balance before bonding: {:?}", balance);

        // Bond 3 NFTs
        let msg = ExecuteMsg::Bond { nft_id: Uint128::from(NFT_ID1) };
        app.execute_contract(Addr::unchecked(AGENT1), staking_contract.addr(), &msg, &[coin(600, NATIVE_DENOM.to_string())]).unwrap();
        let msg = ExecuteMsg::Bond { nft_id: Uint128::from(NFT_ID2) };
        app.execute_contract(Addr::unchecked(AGENT1), staking_contract.addr(), &msg, &[coin(400, NATIVE_DENOM.to_string())]).unwrap();
        let msg = ExecuteMsg::Bond { nft_id: Uint128::from(NFT_ID3) };
        app.execute_contract(Addr::unchecked(AGENT1), staking_contract.addr(), &msg, &[coin(200, NATIVE_DENOM.to_string())]).unwrap();
        let nft_info = get_bonded_by_nft(&app, &staking_contract, NFT_ID1.to_string());
        assert_eq!(nft_info, Uint128::from(600u128)); 

        // let validator1_info = get_validator_info(&app, &staking_contract, VALIDATOR1.into());
        // assert_eq!(validator1_info.bonded, 600u128);        
        // let validator1_info = get_validator_info(&app, &staking_contract, VALIDATOR2.into());
        // assert_eq!(validator1_info.bonded, 400u128);
        // let validator1_info = get_validator_info(&app, &staking_contract, VALIDATOR3.into());
        // assert_eq!(validator1_info.bonded, 200u128);

        // move block year a head and see there are some rewards
        app.update_block(|block| block.time = block.time.plus_seconds(60 * 60 * 24 * 365));
        let total_rewards = query_rewards(&app, staking_contract.addr().as_str(), VALIDATOR1).unwrap();
        assert_eq!(total_rewards,Uint128::from(60u128));

 //       let full_delegation = query_module_delegation(&app, &staking_contract.addr().as_str(), VALIDATOR1);
 //       println!(">>>>>>>>>>>>>> &&&&&&&&&&&&&&&&  full delegation before unbond: {:?}", full_delegation);        
        let msg = ExecuteMsg::Unbond { nft_id: Uint128::from(NFT_ID1), amount: Uint128::from(600u128) };
        app.execute_contract(Addr::unchecked(AGENT1), staking_contract.addr(), &msg, &[]).unwrap();
 //       let full_delegation = query_module_delegation(&app, &staking_contract.addr().as_str(), VALIDATOR1);
 //       println!(">>>>>>>>>>>>>> &&&&&&&&&&&&&&&&  full delegation after unbond: {:?}", full_delegation); 
 //       let full_delegation = query_module_delegation(&app, &staking_contract.addr().as_str(), VALIDATOR2);
//        println!(">>>>>>>>>>>>>> &&&&&&&&&&&&&&&&  full delegation after unbond: {:?}", full_delegation); 
 //       let balance = get_balance(&app, AGENT1.to_string(), NATIVE_DENOM.to_string());
 //       println!(">>>>>>>>>>>>>> AGENT1 balance after bonding: {:?}", balance);

        // let bonded_validator = get_bonded_on_validator(&app, &staking_contract, VALIDATOR1).unwrap();
        // assert_eq!(bonded_validator, Uint128::from(300u128));
        // let validator1_info = get_validator_info(&app, &staking_contract, VALIDATOR1.into());
        // assert_eq!(validator1_info.bonded, 300u128);        
        // let validator1_info = get_validator_info(&app, &staking_contract, VALIDATOR2.into());
        // assert_eq!(validator1_info.bonded, 100u128);
        // let validator1_info = get_validator_info(&app, &staking_contract, VALIDATOR3.into());
        // assert_eq!(validator1_info.bonded, 200u128);


        // 
        app.update_block(|block| block.time = block.time.plus_seconds(60 * 60 * 24 * 7 ));

        //After unbonding period we can claim.
        let msg = ExecuteMsg::Claim { nft_id: Uint128::from(NFT_ID1), sender: AGENT1.to_string(), amount: Uint128::from(600u128) };
        app.execute_contract(Addr::unchecked(AGENT1), staking_contract.addr(), &msg, &[]).unwrap();

        let balance = get_balance(&app, AGENT1.to_string(), NATIVE_DENOM.to_string());
        println!(">>>>>>>>>>>>>> AGENT1 balance after claiming: {:?}", balance);



        // // Unbond NFT_ID1 that bonded 600 
        // let msg = ExecuteMsg::CollectAngelRewards {  };
        // app.execute_contract(Addr::unchecked(MANAGER1), staking_contract.addr(), &msg, &[]).unwrap();

        // Make sure we withdrew the rewards and there are none left.
        // let no_rewards = query_rewards(&app, staking_contract.addr().as_str(), VALIDATOR1);
        // assert!(no_rewards.is_none());

        // //Unbond NFT_ID1 that bonded 600 
        // let msg = ExecuteMsg::Unbond { nft_id: Uint128::from(NFT_ID1), amount: Uint128::from(600u128) };
        // app.execute_contract(Addr::unchecked(AGENT1), staking_contract.addr(), &msg, &[]).unwrap();
    
        

        // //move block year a head
        // app.update_block(|block| block.time = block.time.plus_seconds(60 * 60 * 24 * 365));






        
    }

}