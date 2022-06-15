from .bot.Bot import Bot



mnemonic_key= "escape idea attack owner tiny prepare blind approve cute gentle hidden student knife loyal laundry wreck unlock bunker donor defy sunset immune brief stamp"
bot = Bot("testnet", mnemonic_key)

print(bot.get_wallet())