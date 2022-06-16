from asyncio import constants
from ..bot.Bot import Bot
from ..bot.Pair import Pair
from ..bot.Token import Token
from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys, json, base64

load_dotenv() 
network = "localterra"

deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()
print(deployer.key.acc_address)
user2 = bot.get_lt_wallet("test2")

# deploy token0 and token1 
usd = Token(network, deployer_key, "USD", [(deployer.key.acc_address, "1000"),(user2.key.acc_address, "1000")], deployer.key.acc_address)
sca = Token(network, deployer_key, "SCA", [(deployer.key.acc_address, "1000"),(user2.key.acc_address, "1000")], deployer.key.acc_address)

# deploy pair and lp token
pair = Pair("localterra", deployer_key, repr(sca), repr(usd), "50")
lp = Token(network, deployer_key, "LLP", [(deployer.key.acc_address, "100")], repr(pair))
pair.set_lp_token(repr(lp))

print("\n ==> provide liquidity ")
usd.increase_allowance(deployer, repr(pair), "1000")
sca.increase_allowance(deployer, repr(pair), "1000")

# provide liquidiy 
pair.add_liquid(deployer, "100", "100")
pair.get_lp_token_info(deployer.key.acc_address)

usd.increase_allowance(user2, repr(pair), "1000")
sca.increase_allowance(user2, repr(pair), "1000")

pair.add_liquid(user2, "200", "200")
pair.get_lp_token_info(deployer.key.acc_address)
pair.get_reserves()

print("\n ==> Swap")
pair.swap(user2, "200", [repr(sca), repr(usd)])
pair.get_reserves()

sca.get_balance(repr(pair))
usd.get_balance(repr(pair))


pair.swap(user2, "100", [ repr(usd),repr(sca)])
pair.get_reserves()

sca.get_balance(repr(pair))
usd.get_balance(repr(pair))


sys.exit()
# deploy lp token 


