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

######## WORKING FLOW ##############

###     INIT MINT CONTRACT
mint = Mint(network, deployer_key)

###      INIT TRADING TOKENS
sca = Token(network, deployer_key, "GOLD", [], repr(mint))
usd = Token(network, deployer_key, "USD", [(deployer.key.acc_address, "100000000"),(user2.key.acc_address, "100000000")], deployer.key.acc_address)

###   INIT ORACLE CONTRAcT + set price: 1 gold =  2 usd
oracle = Oracle(network, deployer_key, "1000000")
oracle.set_price(deployer, repr(sca), "2000000")

### INIT PAIR #### 
pair = Pair(network, deployer_key, repr(sca), repr(usd), "50")
llp = Token(network, deployer_key, "LLP", [], repr(pair))
pair.set_lp_token(repr(llp))

## Setting mint contract
mint.set_asset(deployer, repr(oracle), repr(pair), repr(sca), repr(usd), "2000000", "1000000")

# user 2 mint new Gold 
usd.increase_allowance(user2, repr(mint), "1000")
mint.open_position(user2, "1000", "1500000")
sca.get_balance(user2.key.acc_address)


# user 2 provide liquidity 
usd.increase_allowance(user2, repr(pair), "666")
sca.increase_allowance(user2, repr(pair), "333")
pair.add_liquid(user2, "333", "666")

### ACTION WITH MINTERS
mint.get_sca_pool_price()
mint.get_sca_oracle_price()