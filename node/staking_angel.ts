import { CosmWasmClient, SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, Coin } from "cosmwasm";

import * as fs from 'fs';
import axios from 'axios';


const rpcEndpoint = "https://juno-testnet-rpc.polkachu.com/";

// const config = {
//     chainId: "uni-3",
//     rpcEndpoint: rpcEndpoint,
//     prefix: "juno",
// };


const agent_wasm = fs.readFileSync("../artifacts/agent.wasm");
const nft_wasm = fs.readFileSync("../artifacts/nft.wasm");
const staking_wasm = fs.readFileSync("../artifacts/staking.wasm");

const mnemonic = "between anchor impact pact lazy custom rookie load ride ramp piece pony"; // juno1u0h22tk6hrtvulfhq22pcrz5fj9c0cnhqelcpx
const prefix = "juno";

const agent_code_id = 4192;
const nft_code_id = 4193;
const staking_code_id = 4194;

const treasury_address = "juno1vlffrn56q0z06wj95k2u5m2hhl78r5w6hj05rh9gdfn6knsyt08qluruw7";
//multisig1_address const mnemonic = "prosper twelve menu smile canoe vacant stool moment rough weird avoid visual"; // juno1avqzvtvvxv67fje267y8zx9c65838nmjyrqgsh
//multisig2_address const mnemonic = "olympic multiply song tuna estate live fly stomach upon text test birth"; // juno19g70wvrnzavaw03d9szxk47n3aeeettchtzanq
//multisig3_address const mnemonic = "squirrel cube entry gas then dignity lens very rigid duty shrimp moment"; // juno12dyghgkfh4vfmxqeh9y7sdw78zzeksll2aq2fs

const agent_contract_address = "juno136qxeg4l9s02m4hqcnw9p0nfxvejg2at4s35jc7f658v856u9ndshxzcnw"; 
const admin_address = "juno1e8q0t7lt5hnxjk7g5ec5f7t35x6jymxmunnqpd";
const manager_address= "juno1urcl7hkga3j9ek27lvwx784ax9452cn9492826";
const nft_contract_address = "juno1qmyuzkhhx2ucaxels8kmdgnj4y3sgfxhm28q09q56ew3tfmxfrhq6fc94z";
const staking_contract_address = "juno1msj74sh4mxzs9a8tycctl6pp6g42ywnxram29hjtpvmcqccrkczq4es66a";

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

describe("Cosmwasm Template Tests", () => {
    xit("Generate Wallet", async () => {
        let wallet = await Secp256k1HdWallet.generate(12);
        console.log(wallet.mnemonic);
    });

    xit("Get Testnet Tokens", async () => {
        //let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno' });
        //console.log(await wallet.getAccounts());
        console.log(await getAddress(mnemonic));
        try {
            let res = await axios.post("https://faucet.uni.juno.deuslabs.fi/credit", { "denom": "ujunox", "address": await getAddress(mnemonic) });
            console.log(res);
        } catch (e) {
            console.log(e);
        }
    }).timeout(100000);

    xit("Send Testnet Tokens", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let coin:Coin = {denom: "ujunox", amount: "3000000"};
        client.sendTokens(await getAddress(mnemonic), "juno1jjeaun6mlrtv0wzfpt9u57hx6keqsvv7ltuj4j", [coin], "auto");
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

    it("Query nft and staking contract_address from agent_contract ", async() => {
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
   
});