from ...bot.Bot import Bot
from ...bot.Oracle import Oracle
from ...bot.Token import Token 
from ...bot.Mint import Mint 
from ...bot.Controller import Controller

from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys

load_dotenv()

ORACLE_CONTRACT_ADDR = os.getenv('ORACLE_CONTRACT_ADDR')
PAIR_CONTRACT_ADDR = os.getenv("PAIR_CONTRACT_ADDR")
SCA_CONTRACT_ADDR = os.getenv('SCA_CONTRACT_ADDR')
USD_CONTRACT_ADDR = os.getenv('USD_CONTRACT_ADDR')
MINT_CONTRACT_ADDR = os.getenv('MINT_CONTRACT_ADDR')
CONTROLLER_CONTRACT_ADDR = os.getenv("CONTROLLER_CONTRACT_ADDR")

network = "testnet"
deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()


controller = Controller(network, deployer_key, CONTROLLER_CONTRACT_ADDR)



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