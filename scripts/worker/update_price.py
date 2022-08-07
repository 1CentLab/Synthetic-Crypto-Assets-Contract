from ..bot.Bot import Bot
from ..bot.Oracle import Oracle
from ..bot.Mint import Mint 

from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys, time

load_dotenv() 
network = "testnet"
ORACLE_CONTRACT_ADDR = os.getenv('ORACLE_CONTRACT_ADDR')
SCA_CONTRACT_ADDR = os.getenv('SCA_CONTRACT_ADDR')
MINT_CONTRACT_ADDR = os.getenv('MINT_CONTRACT_ADDR')

deployer_key = os.environ.get("MNEMONIC_KEY")

bot = Bot(network, deployer_key)
deployer = bot.get_deployer()
print(deployer.key.acc_address)

oracle = Oracle(network, deployer_key, None, ORACLE_CONTRACT_ADDR)
mint = Mint(network, deployer_key, None, MINT_CONTRACT_ADDR)


prices = [250, 172, 163, 185, 190]
multiplier = 1000000

i = 0


random = False 
max_range = 0.5
while True:
    cprice = prices[i] * multiplier;  
    oracle.set_price(deployer, SCA_CONTRACT_ADDR, str(cprice))
    mint.mass_update(deployer) 

    i+= 1
    if i == len(prices):
        i = 0

    time.sleep(5)