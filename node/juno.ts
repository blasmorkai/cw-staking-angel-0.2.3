import { CosmWasmClient, SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, Coin } from "cosmwasm";

import * as fs from 'fs';
import axios from 'axios';


// const rpcEndpoint = "https://juno-testnet-rpc.polkachu.com/";
//const rpcEndpoint = "https://ares-rpc.reece.sh/";       //v13
const rpcEndpoint = "https://uni-rpc.reece.sh/";       //uni6

// const config = {
//     chainId: "ares-1",
//     rpcEndpoint: rpcEndpoint,
//     prefix: "juno",
// };


const agent_wasm = fs.readFileSync("../artifacts/agent.wasm");
const nft_wasm = fs.readFileSync("../artifacts/nft.wasm");
const staking_wasm = fs.readFileSync("../artifacts/staking.wasm");

const mnemonic = "between anchor impact pact lazy custom rookie load ride ramp piece pony"; // juno1u0h22tk6hrtvulfhq22pcrz5fj9c0cnhqelcpx
const prefix = "juno";


// const agent_code_id = 6;   // v13 code IDs
// const nft_code_id = 7;
// const staking_code_id = 8;
const agent_code_id = 206;
const nft_code_id = 207;
const staking_code_id = 208;

//const treasury_address = "juno1vlffrn56q0z06wj95k2u5m2hhl78r5w6hj05rh9gdfn6knsyt08qluruw7";
//multisig1_address const mnemonic = "prosper twelve menu smile canoe vacant stool moment rough weird avoid visual"; // juno1avqzvtvvxv67fje267y8zx9c65838nmjyrqgsh
//multisig2_address const mnemonic = "olympic multiply song tuna estate live fly stomach upon text test birth"; // juno19g70wvrnzavaw03d9szxk47n3aeeettchtzanq
//multisig3_address const mnemonic = "squirrel cube entry gas then dignity lens very rigid duty shrimp moment"; // juno12dyghgkfh4vfmxqeh9y7sdw78zzeksll2aq2fs

const mnemonic_treasury = "spread language dumb prosper mandate comic abuse tiger jewel avocado salt they"; // juno1w9w3w7qemzqfc6ef3aqt6ejx09r34wa5lsrn2s
const mnemonic_admin = "worry unveil bomb music pact final odor roof document excuse amazing flag"; // juno1e8q0t7lt5hnxjk7g5ec5f7t35x6jymxmunnqpd
const mnemonic_manager = "furnace best mimic know mixed december multiply airport giant donkey ostrich siren"; // juno1urcl7hkga3j9ek27lvwx784ax9452cn9492826


const admin_address = "juno1e8q0t7lt5hnxjk7g5ec5f7t35x6jymxmunnqpd";
const manager_address= "juno1urcl7hkga3j9ek27lvwx784ax9452cn9492826";
const treasury_address ="juno1w9w3w7qemzqfc6ef3aqt6ejx09r34wa5lsrn2s";
// const agent_contract_address = "juno1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gq0zmt6e";  // v13 contract addresses
// const nft_contract_address = "juno1wn625s4jcmvk0szpl85rj5azkfc6suyvf75q6vrddscjdphtve8sdvm67v";
// const staking_contract_address = "juno1tqwwyth34550lg2437m05mjnjp8w7h5ka7m70jtzpxn4uh2ktsmqjucp8k";
const agent_contract_address = "juno1xc0s9lzk82sj9x6sr3rwgrvtlp7329laseu6s8qqueste0jeshkqf7j8xf"; 
const nft_contract_address = "juno1k6we36nauhc85tulnv9pkg65y3hak3u8fclwq260x00cw3m835gssljrdp";
const staking_contract_address = "juno1zpz0clkuppa5f7gmu7g70nszanee4sy9dzy6fahmyfgfj5e23tzqlvjzj0";

async function setupClient(mnemonic: string, rpc: string, gas: string | undefined): Promise<SigningCosmWasmClient> {
    if (gas === undefined) {
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno'});
        let client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet);
        return client;
    } else {
        let gas_price = GasPrice.fromString(gas);
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno' });
        let client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet, { gasPrice: gas_price });
        return client;
    }
}

async function getAddress(mnemonic:string) {
    let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: prefix });
    let accounts = await wallet.getAccounts();
    return accounts[0].address;
}

async function getBalance(mnemonic:string) {
    let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
    let address_user = await getAddress(mnemonic);
    let balance_user = await client.getBalance(address_user, "ujunox");
    console.log("BALANCE user address: ");   
    console.log(balance_user);
    let address_manager = await getAddress(mnemonic_manager);
    let balance_manager = await client.getBalance(address_manager, "ujunox"); 
    console.log("BALANCE manager address: ");     
    console.log(balance_manager);
    let address_treasury = await getAddress(mnemonic_treasury);
    let balance_treasury = await client.getBalance(address_treasury, "ujunox"); 
    console.log("BALANCE treasury address: ");    
    console.log(balance_treasury);    
    return balance_user;
}

describe("Cosmwasm Template Tests", () => {
    xit("Generate Wallet", async () => {
        let wallet = await Secp256k1HdWallet.generate(12);
        console.log(wallet.mnemonic);
        let wallet_prefix = await Secp256k1HdWallet.fromMnemonic(wallet.mnemonic, { prefix: prefix });
        let accounts = await wallet_prefix.getAccounts();
        console.log(accounts[0].address);
    });

    xit("Get Balance", async () => {
        let balance = getBalance(mnemonic);
    });


    xit("Get Testnet Tokens", async () => {
        //let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno' });
        //console.log(await wallet.getAccounts());
        //         Faucet (10junox per 6 hours)
        // http://135.181.85.92:3000/ares-1/<address>
        console.log(await getAddress(mnemonic));
        try {
//            let res = await axios.post("https://faucet.uni.juno.deuslabs.fi/credit", { "denom": "ujunox", "address": await getAddress(mnemonic) });
            let res = await axios.post("http://135.181.85.92:3000/ares-1", { "denom": "ujunox", "address": await getAddress(mnemonic) });
            console.log(res);
        } catch (e) {
            console.log(e);
        }
    }).timeout(100000);

    xit("Send Testnet Tokens", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let coin:Coin = {denom: "ujunox", amount: "500000"};
        client.sendTokens(await getAddress(mnemonic), manager_address, [coin], "auto");
    }).timeout(100000);

    xit("Upload agent_wasm to testnet", async () => {
        //upload NFT contract to testnet twice and get two code_id's
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let sender = await getAddress(mnemonic);
        let res = await client.upload(sender, agent_wasm, "auto", undefined);
        console.log(res);
    }).timeout(100000);

    xit("Upload nft to testnet", async () => {
        //upload NFT contract to testnet twice and get two code_id's
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let sender = await getAddress(mnemonic);
        let res = await client.upload(sender, nft_wasm, "auto", undefined);
        console.log(res);
    }).timeout(100000);

    xit("Upload staking to testnet", async () => {
        //upload NFT contract to testnet twice and get two code_id's
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let sender = await getAddress(mnemonic);
        let res = await client.upload(sender, staking_wasm, "auto", undefined);
        console.log(res);
    }).timeout(100000);

    // pub struct InstantiateMsg {
    //     pub nft_code_id: u64,	
    //     pub staking_code_id: u64, 
    //     pub admin: String,
    //     pub manager: String,
    //     pub treasury: String,
    //  }

    xit("Instantiate agent on testnet", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let sender = await getAddress(mnemonic);
        let res = await client.instantiate(
            sender,
            agent_code_id,
            {
                    nft_code_id: nft_code_id,
                    staking_code_id: staking_code_id,
                    admin: admin_address,
                    manager: manager_address,
                    treasury: treasury_address,
            },
            "angel-staking",
            "auto",
            { admin: admin_address }
        );
        console.log(res);

        for (let i = 0; i<res.logs[0].events.length; i++) {
            console.log("------------EVENTS[%s]-----------------",i);
            console.log(res.logs[0].events[i]);          
        };
    }).timeout(100000);

    xit("Query nft and staking contract_address from agent_contract ", async() => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        // let res = await client.queryContractSmart(contract_cw20_address, {all_allowances:{owner:minter_addr}});
        let query_staking_contract_address = await client.queryContractSmart(agent_contract_address, {get_staking_adress:{}});
        console.log("### Staking contract address stored on agent contract: "+ query_staking_contract_address);
        let query_nft_contract_address = await client.queryContractSmart(agent_contract_address, {get_n_f_t_adress:{}});
        console.log("### NFT contract address stored on agent contract: "+query_nft_contract_address);
        let bonded_tokens = await client.queryContractSmart(query_staking_contract_address, {total_bonded:{}});
        console.log("### Total Bonded on staking contract: "+bonded_tokens);
        let num_nft_tokens = await client.queryContractSmart(query_nft_contract_address, {num_tokens:{}});
        console.log("### Total nft/tokens on nft contract: "+num_nft_tokens.count);
    }).timeout(100000);

    //AddValidator {address: String, bond_denom: String, unbonding_period: Duration},

    xit("Add Validator to staking contract", async() => {
        let client = await setupClient(mnemonic_manager, rpcEndpoint, "0.025ujunox");
        let manager = await getAddress(mnemonic_manager);
        //let validator_address = "junovaloper1zcwdza44mgw0f2esmchc9r32rzzw9n7cttd6rx";  // SparkIBC
        //let validator_address = "junovaloper1t30jjapppmrjeky60sxeeakgqzlkaqk8xn4h05";  // OniValidator  juno1t30jjapppmrjeky60sxeeakgqzlkaqk8ewrc5d
        let validator_address = "junovaloper1jrxluc6weuzpqcm5auxgzvdg3u2alft7m24m67";  // WhisperNode  juno1jrxluc6weuzpqcm5auxgzvdg3u2alft7yhr5p8

        let res = await client.execute(
            manager, staking_contract_address, {
               add_validator : { 
                   address: validator_address, 
                   bond_denom: "ujunox", 
                   unbonding_period: { time: 10 },
               }
           }, 
           "auto", "", []
       );
        // let res = await client.execute(
        //     await getAddress(mnemonic), staking_contract_address, {add_validator : { address: "", bond_denom: "ujunox", unbonding_period: "Message"}}, "auto", "", [{amount: "1000", denom: "ujunox"}]
        // );
        console.log(res);

        for (let i = 0; i<res.logs[0].events.length; i++) {
            console.log("------------EVENTS[%s]-----------------",i);
            console.log(res.logs[0].events[i]);          
        }
     }).timeout(20000);

});