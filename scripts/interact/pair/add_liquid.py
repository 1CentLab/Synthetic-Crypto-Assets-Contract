from ...bot.Bot import Bot
from ...bot.Oracle import Oracle
from ...bot.Token import Token 
from ...bot.Mint import Mint
from ...bot.Pair import Pair

from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys, time

load_dotenv()

ORACLE_CONTRACT_ADDR = os.getenv('ORACLE_CONTRACT_ADDR')
SCA_CONTRACT_ADDR = os.getenv('SCA_CONTRACT_ADDR')
USD_CONTRACT_ADDR = os.getenv('USD_CONTRACT_ADDR')
MINT_CONTRACT_ADDR = os.getenv('MINT_CONTRACT_ADDR')
PAIR_CONTRACT_ADDR = os.getenv("PAIR_CONTRACT_ADDR")

network = "testnet"
deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()


usd = Token(network, deployer_key, None, [], None, USD_CONTRACT_ADDR)
sca = Token(network, deployer_key, None, [], None, SCA_CONTRACT_ADDR)
pair = Pair(network, deployer_key, None, None, None, PAIR_CONTRACT_ADDR)


usd_amount = "200000000"
sca_amount = "100000000"
usd.increase_allowance(deployer, PAIR_CONTRACT_ADDR, usd_amount)

time.sleep(2)
sca.increase_allowance(deployer, PAIR_CONTRACT_ADDR, sca_amount)
time.sleep(2)

pair.add_liquid(deployer, sca_amount, usd_amount)

