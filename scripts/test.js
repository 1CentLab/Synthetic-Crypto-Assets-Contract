const Bot = require("./bot/bot");

const main = async()=>{
    let bot = new Bot()
    await bot.init()
    let wallet = bot.getDeployer();
    console.log(wallet.key.accAddress)
    await bot.storeCode("oracle")

}

main().catch(error => console.log(error));