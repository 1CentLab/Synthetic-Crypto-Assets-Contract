from ..bot.Bot import Bot
from ..bot.Oracle import Oracle
from ..bot.Token import Token
from ..bot.Pair import Pair
from ..bot.Mint import Mint
from ..bot.Controller import Controller
from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys

load_dotenv() 
network = "testnet"

deployer_key = os.environ.get("MNEMONIC_KEY")

bot = Bot(network, deployer_key)
deployer = bot.get_deployer()

#user2 = bot.get_lt_wallet("test2")


CONTROLLER_CONTRACT_ADDR = "terra1quthsmpt03f4fa6zu374cvtgxgtmu8wpaawf5l7amdxyqzsz8avqryn0fk"
MINT_CONTRACT_ADDR= "terra1llwnn473zegsev05xxtn8xqm7tu8xh5h9c9v5eaymda922gwtvvsu7v5v3"
SCA_CONTRACT_ADDR= "terra15c6l234hj7aram3zjgvxn74f4gyffhz3ms6a966vt68d53kqlcls0ax487"
USD_CONTRACT_ADDR= "terra1807udrvqapue6p907xwzax9x05lst67w78ph4a234e49nn9u5rksyup45m"
ORACLE_CONTRACT_ADDR=  "terra1qjp298de4dsplwluzm09zt5s7x09g4m62zp6r4krvsm2h4g8faqspll9cs"
PAIR_CONTRACT_ADDR = "terra1pq0qk9808f9afdyz8gw6d9e4c552dl0tk5mg5eg4crnvcmmwpnqq3fwtt5"
LLP_CONTRACT_ADDR = "terra124qf54wktgtv597zy3zlfav4k2jzklhkrxxqzrcfy9tv47x9cp7s3wzu4v"

######## WORKING FLOW ##############
print("\n============> INIT CONTROLLER  =================>")
controller = Controller(network, deployer_key,CONTROLLER_CONTRACT_ADDR)


print("\n============> INIT MINT  =================>")
mint = Mint(network, deployer_key, repr(controller))

print("\n============> INIT TRADING TOKEN  =================>")
sca = Token(network, deployer_key, "GOLD", [], repr(mint))
usd = Token(network, deployer_key, "USD", [(deployer.key.acc_address, "1000000000000")], deployer.key.acc_address)

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


sys.exit()
print("\n============> DEPLOYER MINT NEW GOLD  =================>")
usd.increase_allowance(deployer, repr(mint), "4000000")
mint.open_position(deployer, "1000000", "2000000")  ## open 1000$ position, ratio: 200%
sca.get_balance(deployer.key.acc_address)
position0=mint.get_position(deployer.key.acc_address)

print("\n============> DEPLOYER PROVIDE LIQUIDITY  =================>")
sca.increase_allowance(deployer, repr(pair), position0['debt'])
usd.increase_allowance(deployer, repr(pair), str(int(position0['debt'])* 2))
pair.add_liquid(deployer, position0['debt'], str(int(position0['debt'])* 2))



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



print("\n============> USER2 BUY AUCTION =================>")

pair.get_reserves() # 250000 - 500000 
usd.increase_allowance(user2, repr(pair),"400000")
pair.swap(user2, "400000", [repr(usd), repr(sca)])
pair.get_reserves() # 142046 - 880000 

user2_sca_balance = sca.get_balance(user2.key.acc_address)['balance']
usd.get_balance(user2.key.acc_address)
sca.get_balance(deployer.key.acc_address)
usd.get_balance(deployer.key.acc_address)
controller.get_asset_state(repr(sca), repr(usd))
sca.increase_allowance(user2, repr(controller), user2_sca_balance)

sca.get_balance(repr(controller))
sca.get_balance(user2.key.acc_address)
usd.get_balance(repr(controller))
usd.get_balance(user2.key.acc_address)
controller.buy_auction(user2, repr(sca), repr(usd), user2_sca_balance)
controller.get_asset_state(repr(sca), repr(usd))

sca.get_balance(repr(controller))
sca.get_balance(user2.key.acc_address)
usd.get_balance(repr(controller))
usd.get_balance(user2.key.acc_address)