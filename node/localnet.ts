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

const agent_code_id = 1;
const nft_code_id = 2;
const staking_code_id = 3;

const admin_address = "juno1mk6fa2nmzqqqgqg9w6ppgeak50xx4hvh66umce";
const manager_address= "juno1d7urzwkzglv390xzjcuztg3tzxkpkawyj56k8r";  
const treasury_address = "juno1ctchndj6zh8eq8n47ka59rdv7j4v0jxmrjh477";

const agent_wasm = fs.readFileSync("../artifacts/agent.wasm");
const nft_wasm = fs.readFileSync("../artifacts/nft.wasm");
const staking_wasm = fs.readFileSync("../artifacts/staking.wasm");

const agent_contract_address = "juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8"; 
const nft_contract_address = "juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p";
const staking_contract_address = "juno17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgszu8fr9";

//const validator_address = "AF1CB27EDC078F81E11BA0596DC1AAAFD2A9E74A";
const validator_address = "junovaloper1hj5fveer5cjtn4wd6wstzugjfdxzl0xp0r8xsx";


// juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl
const mnemonic = "decorate bright ozone fork gallery riot bus exhaust worth way bone indoor calm squirrel merry zero scheme cotton until shop any excess stage laundry"; 
const prefix_address = "juno";

// juno1mk6fa2nmzqqqgqg9w6ppgeak50xx4hvh66umce
const mnemonic_admin = "turtle caught sponsor draft tower eye wise foil stove swing cable pudding key wave leg bless rain lab sunny level second salad bid blush";
// juno1d7urzwkzglv390xzjcuztg3tzxkpkawyj56k8r
const mnemonic_manager = "pool novel odor capital merry process copy imitate struggle steak remain scheme tank beach universe issue robot second skill immune spawn script street adapt";
// juno1ctchndj6zh8eq8n47ka59rdv7j4v0jxmrjh477
const mnemonic_treasury = "robust wear please dress parrot ridge cannon venue outdoor what venue party deposit plate swift play master chase more cake address clutch adult come";

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
    let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: prefix_address });
    let accounts = await wallet.getAccounts();
    console.log("Address: " + accounts[0].address);
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
        let wallet = await Secp256k1HdWallet.generate(24);
        console.log(wallet.mnemonic);
        let sender = await getAddress(wallet.mnemonic);
        console.log(sender);
    });

    xit("Balance Testnet Tokens", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025uosmo");
        let address = await getAddress(mnemonic);
        let res = await client.getBalance(address, "ujuno");
        console.log("Balance for " + address + " : "+ res.amount + " " + res.denom);
        let staking_contract_balance = await client.getBalance(staking_contract_address, "ujuno");
        console.log("Balance for staking contract : "+ staking_contract_balance.amount + " " + staking_contract_balance.denom);  
    }).timeout(100000);

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


    xit("Add Validator to staking contract", async() => {
        let data = await getAccountFromMnemonic(mnemonic_manager, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let manager = await getAddress(mnemonic_manager);
        let res = await client.execute(
             manager, staking_contract_address, {
                add_validator : { 
                    address: validator_address, 
                    bond_denom: "ujuno", 
                    unbonding_period: { time: 10 },
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

     xit("Bond Check", async() => {
        let data = await getAccountFromMnemonic(mnemonic_manager, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let manager = await getAddress(mnemonic_manager);
        let res = await client.execute(
             manager, staking_contract_address, {
                bond_check : { }
            }, 
            "auto", "", []
        );
        //console.log(res);

        for (let i = 0; i<res.logs[0].events.length; i++) {
            console.log("------------EVENTS[%s]-----------------",i);
            console.log(res.logs[0].events[i]);          
        }
     }).timeout(20000); 


     xit("Bond some tokens and get an NFT back", async() => {
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let user = await getAddress(mnemonic);
        let res = await client.execute(
             user, agent_contract_address, {
                bond : { }
            }, 
            "auto", "", [{amount: "1000000", denom: "ujuno"}]
        );
        //console.log(res);

        for (let i = 0; i<res.logs[0].events.length; i++) {
            console.log("------------EVENTS[%s]-----------------",i);
            console.log(res.logs[0].events[i]);          
        }
     }).timeout(20000);   

     xit("ReBond some tokens on the nft 0", async() => {
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let user = await getAddress(mnemonic);
        let res = await client.execute(
             user, agent_contract_address, {
                bond : { nft_id: "0" }
            }, 
            "auto", "", [{amount: "3000", denom: "ujuno"}]
        );
        //console.log(res);

        for (let i = 0; i<res.logs[0].events.length; i++) {
            console.log("------------EVENTS[%s]-----------------",i);
            console.log(res.logs[0].events[i]);          
        }
     }).timeout(20000);  

     xit("UnBond tokens from nft 1", async() => {
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let user = await getAddress(mnemonic);
        let res = await client.execute(
             user, agent_contract_address, {
                unbond : { nft_id: "1" }
            }, 
            "auto", "", []
        );
        //console.log(res);

        for (let i = 0; i<res.logs[0].events.length; i++) {
            console.log("------------EVENTS[%s]-----------------",i);
            console.log(res.logs[0].events[i]);          
        }
     }).timeout(20000);

     xit("Claim tokens from nft 1", async() => {
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let user = await getAddress(mnemonic);
        let res = await client.execute(
             user, agent_contract_address, {
                claim : { nft_id: "1" }
            }, 
            "auto", "", []
        );
        //console.log(res);

        for (let i = 0; i<res.logs[0].events.length; i++) {
            console.log("------------EVENTS[%s]-----------------",i);
            console.log(res.logs[0].events[i]);          
        }
     }).timeout(20000);

     
     xit("Query nft information ", async() => {
        let data = await getAccountFromMnemonic(mnemonic, "juno"); 
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);
        let sender = await getAddress(mnemonic);
        // let res = await client.queryContractSmart(contract_cw20_address, {all_allowances:{owner:minter_addr}});
        let nft_info = await client.queryContractSmart(nft_contract_address, {all_nft_info:{token_id: "1"}});
        //console.log(nft_info);
        console.log("NFT owner: " + nft_info.access.owner);
        console.log("Tokens bonded:");
        console.log(nft_info.info.extension.native);
        console.log("Status: "+ nft_info.info.extension.status);
        console.log("-------------------------"); 
        // BondedByNFT { nft_id }
        let nft_bonded_staking_contract = await client.queryContractSmart(staking_contract_address, {bonded_by_n_f_t:{nft_id: "0"}});
        console.log("Tokens bonded on staking contract by nft 0 : "+ nft_bonded_staking_contract);
    }).timeout(100000);


});

