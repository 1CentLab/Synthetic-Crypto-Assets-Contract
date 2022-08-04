from ...bot.Bot import Bot
from ...bot.Oracle import Oracle
from ...bot.Token import Token 
from ...bot.Mint import Mint 

from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys

load_dotenv()

ORACLE_CONTRACT_ADDR = os.getenv('ORACLE_CONTRACT_ADDR')
SCA_CONTRACT_ADDR = os.getenv('SCA_CONTRACT_ADDR')
USD_CONTRACT_ADDR = os.getenv('USD_CONTRACT_ADDR')
MINT_CONTRACT_ADDR = os.getenv('MINT_CONTRACT_ADDR')

network = "testnet"
deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()


mint = Mint(network, deployer_key, None, MINT_CONTRACT_ADDR)
positions = mint.get_all_positions()


for position in positions:
    print(f"=============== Position for {position} ===================")
    mint.get_position(position) 
    print("\n \n ")