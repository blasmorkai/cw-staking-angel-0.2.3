import { CosmWasmClient, SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, Coin } from "cosmwasm";

import * as fs from 'fs';

//const rpcEndpoint = "127.0.0.1:26657";
//const rpcEndpoint = "tcp://0.0.0.0:26657";
//const rpcEndpoint = "tcp://127.0.0.1:26657";
const rpcEndpoint = "http://localhost:26657/";
//rpc_address	"tcp://0.0.0.0:26657"

// const config = {
//     chainId: "local-1",
//     rpcEndpoint: rpcEndpoint,
//     prefix: "ujuno",
// };

const config = {
    chainId: "local-1",
    rpcEndpoint: rpcEndpoint,
    prefix: "ujuno",
    gasPrice: GasPrice.fromString("0.03ujuno"),
};

const agent_code_id = 2;
const nft_code_id = 3;
const staking_code_id = 4;

const admin_address = "juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk";
const manager_address= "juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk";  // "wealth flavor believe regret funny network recall kiss grape useless pepper cram hint member few certain unveil rather brick bargain curious require crowd raise"
const treasury_address = "juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk";

const agent_wasm = fs.readFileSync("../artifacts/agent.wasm");
const nft_wasm = fs.readFileSync("../artifacts/nft.wasm");
const staking_wasm = fs.readFileSync("../artifacts/staking.wasm");

const agent_contract_address = "juno1aakfpghcanxtc45gpqlx8j3rq0zcpyf49qmhm9mdjrfx036h4z5squu0w2"; 
const nft_contract_address = "juno1qwlgtx52gsdu7dtp0cekka5zehdl0uj3fhp9acg325fvgs8jdzks9z8n5r";
const staking_contract_address = "juno1kj8q8g2pmhnagmfepp9jh9g2mda7gzd0m5zdq0s08ulvac8ck4dqhnemjt";


// juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl
const mnemonic = "decorate bright ozone fork gallery riot bus exhaust worth way bone indoor calm squirrel merry zero scheme cotton until shop any excess stage laundry"; 
const prefix_address = "juno";

async function setupClient(mnemonic: string, rpc: string, gas: string | undefined): Promise<SigningCosmWasmClient> {
    if (gas === undefined) {
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'ujuno'});
        let client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet);
        return client;
    } else {
        let gas_price = GasPrice.fromString(gas);
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'ujuno' });
        let client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet, { gasPrice: gas_price });
        return client;
    }
}

async function getAddress(mnemonic:string) {
    let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: prefix_address });
    let accounts = await wallet.getAccounts();
    console.log(accounts[0].address);
    return accounts[0].address;
}

export const getAccountFromMnemonic = async (mnemonic: any, prefix: string = "cosmos") => {
    let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: prefix });
    const [account] = await wallet.getAccounts();
    return {
        wallet: wallet,
        account: account,
    }
}

describe("Cosmwasm Template Tests", () => {
    xit("Generate Wallet", async () => {
        let wallet = await Secp256k1HdWallet.generate(12);
        console.log(wallet.mnemonic);
    });

    xit("Upload agent_wasm to local testnet", async () => {
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let sender = await getAddress(mnemonic);
        let res = await client.upload(sender, agent_wasm, "auto", undefined);
        console.log(res);
    }).timeout(100000);

    xit("Upload nft_wasm to local testnet", async () => {
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let sender = await getAddress(mnemonic);
        let res = await client.upload(sender, nft_wasm, "auto", undefined);
        console.log(res);
    }).timeout(100000);

    xit("Upload staking_wasm to local testnet", async () => {
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let sender = await getAddress(mnemonic);
        let res = await client.upload(sender, staking_wasm, "auto", undefined);
        console.log(res);
    }).timeout(100000);

    xit("Instantiate agent on testnet", async () => {
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
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
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
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


});

