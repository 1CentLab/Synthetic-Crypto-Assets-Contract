const fetch  = require("isomorphic-fetch");
const { Coins, LCDClient,MnemonicKey , MsgStoreCode} = require("@terra-money/terra.js");
const fs = require("fs")
const {resolve} = require("path")

class Bot{
    async init(){
        const gasPrices = await fetch(
            "https://pisco-api.terra.dev/gas-prices", { redirect: 'follow' }
        );
        const gasPricesJson = await gasPrices.json();
        const gasPricesCoins = new Coins(gasPricesJson);
        this.lcd = new LCDClient({
        URL: "https://pisco-lcd.terra.dev", // Use "https://phoenix-lcd.terra.dev" for prod "http://localhost:1317" for localterra.
        chainID: "pisco-1", // Use "phoenix-1" for production or "localterra".
        gasPrices: gasPricesCoins,
        gasAdjustment: "1.5", // Increase gas price slightly so transactions go through smoothly.
        gas: 100000000,
        });
        
        const mk = new MnemonicKey({
            mnemonic:
              "escape idea attack owner tiny prepare blind approve cute gentle hidden student knife loyal laundry wreck unlock bunker donor defy sunset immune brief stamp",
          });
        this.deployer = this.lcd.wallet(mk)
    }



    async storeCode(wasm_file){ 
        let abpath = resolve(`artifacts/${wasm_file}.wasm`)
        
        const msg = new MsgStoreCode(
            this.deployer.key.accAddress,
            fs.readFileSync(abpath)
        )

        const storeCodeTx = await this.deployer.createAndSignTx({
            msgs: [msg],
          });
        const storeCodeTxResult = await this.lcd.tx.broadcast(storeCodeTx);
        console.log(storeCodeTxResult)
        
        // if (isTxError(storeCodeTxResult)) {
        //     throw new Error(
        //     `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
        //     );
        // }

        // const {
        //     store_code: { code_id },
        //   } = storeCodeTxResult.logs[0].eventsByType;
    }

    getDeployer(){
        return this.deployer
    }
}


module.exports = Bot