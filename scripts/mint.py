from bot.Bot import Bot
from bot.Oracle import Oracle
from bot.Token import Token
from bot.Pair import Pair
from bot.Mint import Mint
from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os

load_dotenv() 
network = "localterra"

deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()
user2 = bot.get_lt_wallet("test2")


# init new mint contract 
mint = Mint(network, deployer_key)

# init new token contract
gold = Token(network, deployer_key, "GOLD", [], repr(mint))
usd = Token(network, deployer_key, "USD", [(deployer.key.acc_address, "1000"),(user2.key.acc_address, "1000")], deployer.key.acc_address)


# init new oracle contract
oracle = Oracle(network, deployer_key, "1000000")
pair = Pair(network, deployer_key, repr(gold), repr(usd), "50")

oracle.set_price(deployer, repr(gold), "20000000")
oracle.get_price(repr(gold))


## Setting mint contract
mint.set_asset(deployer, repr(oracle), repr(pair), repr(gold), repr(usd), "2000000", "1000000")


## user 2 open position 
usd.increase_allowance(user2, repr(mint), "1000")
mint.open_position(user2, "500", "2000000")
mint.open_position(user2, "500", "2000000")
mint.get_position(user2.key.acc_address)

## check balance of sca
gold.get_balance(user2.key.acc_address)

## close position 
gold.increase_allowance(user2, repr(mint), "8")
mint.close_position(user2, "8")
mint.get_position(user2.key.acc_address)


### TODO: handle liquidations + auctions