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


CONTROLLER_CONTRACT_ADDR = "terra1gjs0nnqrpq8sc75u904k490sux0c2jdt6203lfrm9r587yvu4jhshqgqa2"
MINT_CONTRACT_ADDR= "terra12xs760tu6v3nxmug3mmr0pczaetds0s3yna0cj2hpyuyj3a3j8dsyhy0x8"
SCA_CONTRACT_ADDR= "terra1yn7aq220d2yntm6rmzkm2fdfnyttd7qw9y5e3rjj3wtrae8fvudstphknp"
USD_CONTRACT_ADDR= "terra1wppq09rprz8kk7q4rs9s7l3qfqazm27857t8zek396pkd42vkvsqppm87c"
ORACLE_CONTRACT_ADDR=  "terra1mdc45aws0s9uq5wt09rlqqjyc9lea96dhxuflvdh52p0ae49chhqj5m0w8"
PAIR_CONTRACT_ADDR = "terra14jjpezl3cy2va833cgenvyl6kyxtfrkt2uq66gesnq2mn8uy6kusml0eep"
LLP_CONTRACT_ADDR = "terra1j8j088gmmvqqc0k8ud0g4j0h7te47xu7qcr7nr3qcje9hg8jelgsx6hs95"

######## WORKING FLOW ##############
# print("\n============> INIT CONTROLLER  =================>")
# controller = Controller(network, deployer_key)


# print("\n============> INIT MINT  =================>")
# mint = Mint(network, deployer_key, repr(controller))

# print("\n============> INIT TRADING TOKEN  =================>")
# sca = Token(network, deployer_key, "GOLD", [], repr(mint))
# usd = Token(network, deployer_key, "USD", [(deployer.key.acc_address, "1000000000000")], deployer.key.acc_address)

# print("\n============> INIT ORACLE CONTRACT  =================>")
# oracle = Oracle(network, deployer_key, "1000000")
# oracle.set_price(deployer, repr(sca), "2000000")

# print("\n============> INIT PAIR =================>")
# pair = Pair(network, deployer_key, repr(sca), repr(usd), "50")
# llp = Token(network, deployer_key, "LLP", [], repr(pair))
# pair.set_lp_token(repr(llp))


controller = Controller(network, deployer_key, CONTROLLER_CONTRACT_ADDR)
mint = Mint(network, deployer_key, None, MINT_CONTRACT_ADDR)


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