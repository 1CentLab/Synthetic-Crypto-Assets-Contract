from ...bot.Bot import Bot
from ...bot.Oracle import Oracle

from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys

load_dotenv()

ORACLE_CONTRACT_ADDR = os.getenv('ORACLE_CONTRACT_ADDR')
SCA_CONTRACT_ADDR = os.getenv('SCA_CONTRACT_ADDR')

network = "testnet"
deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()


oracle = Oracle(network, deployer_key, None, ORACLE_CONTRACT_ADDR)


oracle.get_price(SCA_CONTRACT_ADDR)
oracle.set_price(deployer, SCA_CONTRACT_ADDR, "1000000")