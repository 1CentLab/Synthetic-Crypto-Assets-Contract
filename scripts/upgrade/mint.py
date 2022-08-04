from ..bot.Bot import Bot
from ..bot.Oracle import Oracle
from ..bot.Token import Token
from ..bot.Pair import Pair
from ..bot.Mint import Mint
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


#mint_id = bot.store_contract("mint")
mint_id = 2753
mint = Mint(network, deployer_key, None, MINT_CONTRACT_ADDR)
mint.migrate( mint_id, {}) 