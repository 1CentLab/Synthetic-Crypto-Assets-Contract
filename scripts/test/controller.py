from ..bot.Bot import Bot
from ..bot.Oracle import Oracle
from ..bot.Token import Token
from ..bot.Pair import Pair
from ..bot.Mint import Mint
from ..bot.Controller import Controller
from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os

load_dotenv() 
network = "localterra"

deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()
user2 = bot.get_lt_wallet("test2")



price = 1000000


######## WORKING FLOW ##############
print("\n============> INIT CONTROLLER  =================>")
controller = Controller(network, deployer_key)


print("\n============> INIT MINT  =================>")
mint = Mint(network, deployer_key, repr(controller))

print("\n============> INIT TRADING TOKEN  =================>")
sca = Token(network, deployer_key, "GOLD", [], repr(mint))
usd = Token(network, deployer_key, "USD", [(deployer.key.acc_address, "100000000"),(user2.key.acc_address, "100000000")], deployer.key.acc_address)

print("\n============> INIT ORACLE CONTRACT  =================>")
oracle = Oracle(network, deployer_key, "1000000")
oracle.set_price(deployer, repr(sca), str(price))

print("\n============> INIT PAIR =================>")
pair = Pair(network, deployer_key, repr(sca), repr(usd), "50")
llp = Token(network, deployer_key, "LLP", [], repr(pair))
pair.set_lp_token(repr(llp))


print("\n ============> SET NEW ASSSET FOR CONTROLLER =================>")
asset = {
    "oracle": repr(oracle),
    "pair": repr(pair),
    "sca": repr(sca),
    "collateral": repr(usd),
    "mcr": "1500000",
    "multiplier": "1000000",
    "premium_rate": "1000000"
}
controller.add_asset(deployer, asset)

print("\n============> SETTING ASSET MINTERS =================>")
mint.set_asset(deployer, asset) # mcr: 150%:  1 gold (10$) => cap collateral: 15$


print("\n============> DEPLOYER MINT NEW GOLD  =================>")
usd.increase_allowance(deployer, repr(mint), "4000000")
mint.open_position(deployer, "10000", "1500000")  ## open 1000$ position, ratio: 200%. Collateral amount / ratio / oracle_price  (ratio >= 150%)
sca.get_balance(deployer.key.acc_address)
position0=mint.get_position(deployer.key.acc_address)

print("\n============> PRICE INCREASE 60%  =================>")
price10 = int(price * 1.3)
oracle.set_price(deployer, repr(sca), str(price10)) 
mint.mass_update(deployer)
mint.get_position(deployer.key.acc_address)


controller.get_asset_state(repr(sca), repr(usd))


