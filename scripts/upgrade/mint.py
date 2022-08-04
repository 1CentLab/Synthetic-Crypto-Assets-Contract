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

MINT_CONTRACT_ADDR = os.environ.get("MINT_CONTRACT_ADDR")

bot = Bot(network, deployer_key)
deployer = bot.get_deployer()

#user2 = bot.get_lt_wallet("test2")



######## WORKING FLOW ##############


mint_id = bot.store_contract("mint")
mint = Mint(network, deployer_key, None, MINT_CONTRACT_ADDR)
mint.migrate( mint_id, {}) 