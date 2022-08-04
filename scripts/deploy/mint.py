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
print(deployer.key.acc_address)
#user2 = bot.get_lt_wallet("test2")


CONTROLLER_CONTRACT_ADDR = "terra1y9dqzsuvmy8kfw254hjvw98m3ea3uhdhtsxv2hwe8398em9cjqpqsayn45"
MINT_CONTRACT_ADDR= "terra17edtnxf6zrxfae0m395kp3jeklf7725d3rrvw7cqmw858q5xv8kqg3yfx0"
SCA_CONTRACT_ADDR= "terra1g7tlk4u0cpysw0u2pgapy384prsseng0p4uenltgrwewh7nu83xqt342kg"
USD_CONTRACT_ADDR= "terra1kwrfftqyz3e4j5wl4amsauucg3fk0kq5qyfwn23l8vqa0vp2jndsk5vccr"
ORACLE_CONTRACT_ADDR=  "terra1etxnrd8gd2964cuhxxh5p7rk730rz9zaa8qr26rusvxhhktwae3qxrqkug"
PAIR_CONTRACT_ADDR = "terra1zur24awggcdyamnyuzmkc79nufjfallz84dygm0wn5aqk6f2rpqqcrau9c"
LLP_CONTRACT_ADDR = "terra1z8g525l0243r53kmzp6dysvshna3lzaa7cvvs0z788yhkpky3plqchut0a"

######## WORKING FLOW ##############
print("\n============> INIT CONTROLLER  =================>")
controller = Controller(network, deployer_key)


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
    "oracle": ORACLE_CONTRACT_ADDR,
    "pair": PAIR_CONTRACT_ADDR,
    "sca": SCA_CONTRACT_ADDR,
    "collateral": USD_CONTRACT_ADDR,
    "mcr": "1500000",
    "multiplier": "1000000",
    "premium_rate": "1000000"
}
controller.add_asset(deployer, asset)

print("\n============> SETTING ASSET MINTERS =================>")
mint.set_asset(deployer, asset)


sys.exit()