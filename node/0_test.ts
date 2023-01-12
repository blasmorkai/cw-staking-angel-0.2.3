import { CosmWasmClient, SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, Coin } from "cosmwasm";

import * as fs from 'fs';
import axios from 'axios';


const rpcEndpoint = "https://juno-testnet-rpc.polkachu.com/";

const config = {
    chainId: "uni-3",
    rpcEndpoint: rpcEndpoint,
    prefix: "juno",
};


const agent_wasm = fs.readFileSync("../artifacts/agent.wasm");
const nft_wasm = fs.readFileSync("../artifacts/nft.wasm");
const staking_wasm = fs.readFileSync("../artifacts/staking.wasm");

const mnemonic = "between anchor impact pact lazy custom rookie load ride ramp piece pony";

const prefix = "juno";

const agent_code_id = 4100;
const nft_code_id = 4101;
const staking_code_id = 4102;

const manager_address = "";
const treasury_address = "";
const agent_address = ""; 


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

    it("Upload staking to testnet", async () => {
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
                    nft_code_id: 4101,
                    staking_code_id: 4102,
                    admin: "",
                    manager: "",
                    treasury: "",
            },
            "angel-staking",
            "auto",
            {admin:sender}
        );
        console.log(res);
    }).timeout(100000);
   
});