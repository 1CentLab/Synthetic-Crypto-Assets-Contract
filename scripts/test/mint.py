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
oracle.set_price(deployer, repr(sca), "2000000")

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
    "multiplier": "1000000"
}
controller.add_asset(deployer, asset)

print("\n============> SETTING ASSET MINTERS =================>")
mint.set_asset(deployer, repr(oracle), repr(pair), repr(sca), repr(usd), "1500000", "1000000")

print("\n============> DEPLOYER MINT NEW GOLD  =================>")
usd.increase_allowance(deployer, repr(mint), "4000000")
mint.open_position(deployer, "1000000", "2000000")  ## open 1000$ position, ratio: 200%
sca.get_balance(deployer.key.acc_address)
position0=mint.get_position(deployer.key.acc_address)

print("\n============> DEPLOYER PROVIDE LIQUIDITY  =================>")
sca.increase_allowance(deployer, repr(pair), position0['debt'])
usd.increase_allowance(deployer, repr(pair), str(int(position0['debt'])* 2))
pair.add_liquid(deployer, position0['debt'], str(int(position0['debt'])* 2))

print("\n============> USER 2 MINT NEW GOLD  =================>")
usd.increase_allowance(user2, repr(mint), "1000")
mint.open_position(user2, "1000", "4000000")  ## open 1000$ position, ratio: 200%
sca.get_balance(user2.key.acc_address)
position2=mint.get_position(user2.key.acc_address)


print("\n============> RWA PRICE INCREASES TO 3 =================>")
mint.get_sca_oracle_price()
mint.get_sca_pool_reserves()
oracle.set_price(deployer, repr(sca), "3000000")
mint.get_sca_oracle_price()

mint.mass_update(deployer)
mint.get_position(user2.key.acc_address)
mint.get_position(deployer.key.acc_address)
usd.get_balance(repr(controller))
controller.get_asset_state(repr(sca), repr(usd))




print("\n============> RWA PRICE INCREASES TO 6 =================>")
oracle.set_price(deployer, repr(sca), "6000000")
mint.mass_update(deployer)
mint.get_position(user2.key.acc_address)
mint.get_position(deployer.key.acc_address)
usd.get_balance(repr(controller))

print("\n============> DEPLOYER POSITION HAS LIQUIDATED =================>")
usd.get_balance(repr(controller))
controller.get_asset_state(repr(sca), repr(usd))

print("\n============> DEPLOYER MANUALLY CLOSE POSITION =================>")
sca.increase_allowance(user2, repr(mint), position2["debt"])

usd.get_balance(user2.key.acc_address)
mint.close_position(user2, str(int(int(position2["debt"])/ 5)))
usd.get_balance(user2.key.acc_address)

sca.get_balance(user2.key.acc_address)
controller.get_asset_state(repr(sca), repr(usd))
mint.get_position(user2.key.acc_address)
mint.get_position(deployer.key.acc_address)

controller.get_sca_oracle_price(repr(sca), repr(usd))
controller.get_sca_pool_reserve(repr(sca), repr(usd))