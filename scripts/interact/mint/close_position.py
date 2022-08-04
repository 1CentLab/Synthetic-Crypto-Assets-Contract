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


mint = Mint(network, deployer_key, "USD", MINT_CONTRACT_ADDR)
sca = Token(network, deployer_key, "GOLD", [], None, SCA_CONTRACT_ADDR)

position = mint.get_position(deployer.key.acc_address)

sca.increase_allowance(deployer, repr(mint), position['debt'])
mint.close_position(deployer, position['debt'])
