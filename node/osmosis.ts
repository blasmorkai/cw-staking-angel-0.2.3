import { SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, Coin } from "cosmwasm";

import * as fs from 'fs';
import axios from 'axios';
import { ClientRequest } from "http";
import assert from "assert";


const rpcEndpoint= "https://rpc-test.osmosis.zone";
// const rpcEndpoint = "https://rpc.testnet.osmosis.zone";
// const rpcEndpoint = "https://lcd.osmo-test.ccvalidators.com/";
// const rpcEndpoint = "https://testnet-rest.osmosis.zone/"
const agent_wasm = fs.readFileSync("../artifacts/agent.wasm");
const nft_wasm = fs.readFileSync("../artifacts/nft.wasm");
const staking_wasm = fs.readFileSync("../artifacts/staking.wasm");


const agent_code_id = 5451;
const nft_code_id = 5452;
const staking_code_id = 5453;

const mnemonic ="snake matter broom era animal aspect robust fresh arrest wealth merry attract"; // osmo1t6c0tqwz92jt03g0vrhdegl83mwcjnye6l5ded
const mnemonic_admin = "mesh this vehicle buffalo jazz mechanic edit hill review menu planet like"; // osmo1f9my2kvz3664pdnvxg5ag6wpdvezzvyhf07zqe
const mnemonic_manager = "move neglect federal banner clarify glare disagree bird screen bitter hire connect"; // osmo1r8ph8pnqd9pz3gj5yzsq5lw2jykd6x237dmvs0
const mnemonic_treasury = "brave crop demand kitchen broken soft pipe film aerobic warrior arena strike"; // osmo1n9s48xuaxgrwr2fdtzp8mcs4vuny8jx5caysry
const mnemonic_another = "until hawk beauty virus nephew evolve hood crucial humble now inspire stairs"; // osmo1u3t3y27x502wlv8gnyx5q243sw3yg24kmwgq36

const agent_contract_address = "osmo1npt4krn5upax75c737mugd9x0u8kvjugz59yd4jyt05sznhmj9fqkzsflm"; 
const admin_address = "osmo1f9my2kvz3664pdnvxg5ag6wpdvezzvyhf07zqe";
const manager_address= "osmo1r8ph8pnqd9pz3gj5yzsq5lw2jykd6x237dmvs0";
const treasury_address = "osmo1n9s48xuaxgrwr2fdtzp8mcs4vuny8jx5caysry";
const nft_contract_address = "osmo1kf7qmfzza50rggdwjge7j6djnhpw8m736r8cr55hz0ys4d34tmhss3wtzg";
const staking_contract_address = "osmo1z3n462xldj0whwz2tyws7un6qyydwypfusch4pjm9mqr8vrl2c8sufh6jn";

const validator_address = "osmovaloper1c584m4lq25h83yp6ag8hh4htjr92d954kphp96";


async function setupClient(mnemonic: string, rpc: string, gas: string | undefined): Promise<SigningCosmWasmClient> {
    if (gas === undefined) {
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'osmo'});
        let client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet);
        return client;
    } else {
        let gas_price = GasPrice.fromString(gas);
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'osmo' });
        let client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet, { gasPrice: gas_price });
        return client;
    }
}

async function getAddress(mnemonic: string, prefix: string = 'osmo') {
    let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix });
    let accounts = await wallet.getAccounts();
    return accounts[0].address;
}

async function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

describe("Angel Staking Test", () => {
    xit("Generate Wallet", async () => {
        let wallet = await Secp256k1HdWallet.generate(12);
        console.log(wallet.mnemonic);
        console.log(await getAddress(wallet.mnemonic));
    });

    xit("Get Address", async() => {
        console.log(await getAddress(mnemonic));
    }).timeout(200000);

    xit("Get Testnet Tokens", async () => {
        console.log(await getAddress(mnemonic));
        try {
            let res = await axios.post("https://faucet.osmosis.zone", { "denom": "uosmo", "address": await getAddress(mnemonic) });
            console.log(res);
        } catch (e) {
            console.log(e);
        }
    }).timeout(100000);

    xit("Send Testnet Tokens", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025uosmo");
        let receiver = "";
        let res = await client.sendTokens(await getAddress(mnemonic), receiver, [{denom:"uosmo", amount:"1000000"}], "auto");
        console.log(res);
    }).timeout(100000);

    xit("Balance Testnet Tokens", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025uosmo");
        let res = await client.getBalance(await getAddress(mnemonic), "uosmo");
        console.log(res);  
    }).timeout(100000);

    xit("Upload agent_wasm to testnet", async () => {
        //upload NFT contract to testnet twice and get two code_id's
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025uosmo");
        let sender = await getAddress(mnemonic);
        let res = await client.upload(sender, agent_wasm, "auto", undefined);
        console.log(res);
    }).timeout(100000);

    xit("Upload nft to testnet", async () => {
        //upload NFT contract to testnet twice and get two code_id's
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025uosmo");
        let sender = await getAddress(mnemonic);
        let res = await client.upload(sender, nft_wasm, "auto", undefined);
        console.log(res);
    }).timeout(100000);

    xit("Upload staking to testnet", async () => {
        //upload NFT contract to testnet twice and get two code_id's
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025uosmo");
        let sender = await getAddress(mnemonic);
        let res = await client.upload(sender, staking_wasm, "auto", undefined);
        console.log(res);
    }).timeout(100000);

    xit("Instantiate agent on testnet", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025uosmo");
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
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025uosmo");
        let sender = await getAddress(mnemonic);
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

    xit("Add Validator to staking contract", async() => {
        let client = await setupClient(mnemonic_manager, rpcEndpoint, "0.025uosmo");
        let manager = await getAddress(mnemonic_manager);
        let res = await client.execute(
             manager, staking_contract_address, {
                add_validator : { 
                    address: validator_address, 
                    bond_denom: "osmo",
                    unbonding_period: { time: 1209600 },             // 60 * 60 * 24 * 14
                }
            }, 
            "auto", "", []
        );
        //console.log(res);

        for (let i = 0; i<res.logs[0].events.length; i++) {
            console.log("------------EVENTS[%s]-----------------",i);
            console.log(res.logs[0].events[i]);          
        }
     }).timeout(20000); 


});