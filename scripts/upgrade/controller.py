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

CONTROLLER_CONTRACT_ADDR = os.environ.get("CONTROLLER_CONTRACT_ADDR")

bot = Bot(network, deployer_key)
deployer = bot.get_deployer()

#user2 = bot.get_lt_wallet("test2")

######## WORKING FLOW ##############


controller_id = bot.store_contract("controller")
controller = Controller(network, deployer_key, CONTROLLER_CONTRACT_ADDR)
controller.migrate( controller_id, {}) 