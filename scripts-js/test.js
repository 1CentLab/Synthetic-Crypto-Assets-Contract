const Bot = require("./bot/bot");

const main = async()=>{
    data = '[{"events":[{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgStoreCode"},{"key":"module","value":"wasm"},{"key":"sender","value":"terra19elqj48repnjzwuv935x2q8p0vvwfs4dh9ygqc"}]},{"type":"store_code","attributes":[{"key":"code_id","value":"746"}]}]}]'
    data = JSON.parse(data)
    console.log(data[0]['events'])


    process.exit(1)
    let bot = new Bot()
    await bot.init()
    let wallet = bot.getDeployer();
    console.log(wallet.key.accAddress)
    await bot.storeCode("oracle")

}

main().catch(error => console.log(error));