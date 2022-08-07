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


PAIR_CONTRACT_ADDR = os.environ.get("PAIR_CONTRACT_ADDR")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()

#user2 = bot.get_lt_wallet("test2")



######## WORKING FLOW ##############


bot_id = bot.store_contract("pair")
pair= Pair(network, deployer_key, None, None, None, PAIR_CONTRACT_ADDR)
pair.migrate(2789, {}) 