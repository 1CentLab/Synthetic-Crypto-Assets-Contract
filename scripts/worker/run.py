from hashlib import new
from ..bot.Bot import Bot
from ..bot.Oracle import Oracle
from ..bot.Mint import Mint 
from ..bot.Pair import Pair 
from ..bot.Token import Token

from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys, time
import random
import pandas as pd 

load_dotenv() 
network = "testnet"
ORACLE_CONTRACT_ADDR = os.getenv('ORACLE_CONTRACT_ADDR')
USD_CONTRACT_ADDR = os.getenv("USD_CONTRACT_ADDR")
SCA_CONTRACT_ADDR = os.getenv('SCA_CONTRACT_ADDR')
MINT_CONTRACT_ADDR = os.getenv('MINT_CONTRACT_ADDR')
PAIR_CONTRACT_ADDR = os.getenv("PAIR_CONTRACT_ADDR")


deployer_key = os.environ.get("MNEMONIC_KEY")

bot = Bot(network, deployer_key)
deployer = bot.get_deployer()
print(deployer.key.acc_address)

oracle = Oracle(network, deployer_key, None, ORACLE_CONTRACT_ADDR)
mint = Mint(network, deployer_key, None, MINT_CONTRACT_ADDR)
pair = Pair(network, deployer_key, None, None, None, PAIR_CONTRACT_ADDR)
usd = Token(network, deployer_key, None, [], None, USD_CONTRACT_ADDR)
sca = Token(network, deployer_key, None, [], None, SCA_CONTRACT_ADDR)


usd_balance = usd.get_balance(deployer.key.acc_address)
sca_balance = sca.get_balance(deployer.key.acc_address)

usd.increase_allowance(deployer, repr(pair), usd_balance['balance'])
sca.increase_allowance(deployer, repr(pair), sca_balance['balance'])

def add_reserve(new_price):
    reserves = pair.get_reserves()
    amount0 = int(reserves['reserve0'])
    amount1 = int(reserves['reserve1'])
    price = amount1 /amount0

    ## increase usd liquid
    if price < new_price:
        added_amount = int( amount0 * new_price - amount1)
        
        pair.add_liquid(deployer, "0", str(added_amount)) 
    else: # increase sca liquid
        added_amount = int((amount1 - (amount0 * new_price)) / new_price)
        pair.add_liquid(deployer, str(added_amount), "0")
         

    reserves = pair.get_reserves()
    amount0 = int(reserves['reserve0'])
    amount1 = int(reserves['reserve1'])
    price = amount1 /amount0
    print(f"Price update to: {price} with reserve {amount0}, {amount1}")

prices = [3, 172, 163, 185, 190]
multiplier = 1000000

i = 0

IS_RANDOM = True 
INITIAL_PRICE = prices[0]
new_price= INITIAL_PRICE 

DEVIATION = 3
PRICE_CHANGE = 20

prices_data = []

while True:
    if IS_RANDOM:
        percent_change = random.randint(-PRICE_CHANGE, PRICE_CHANGE)
        old_price= new_price
        new_price = (1+ percent_change/100 ) * old_price

        print(f"==================== TARGET PRICE  {new_price} ===================== ")

        deviation_percent = random.randint(-DEVIATION, DEVIATION)
        on_chain_price = int((new_price * multiplier) * (1 + deviation_percent/ 100) )

        try:
            time.sleep(0.1)
            oracle.set_price(deployer, SCA_CONTRACT_ADDR, str(on_chain_price))
        except:
            print("XXXXXX Executing contract FAIL XXXXXXXXXXX")
            new_price = old_price
            time.sleep(1)
            continue

        try:
            time.sleep(0.1)
            add_reserve(new_price)
            time.sleep(0.1)
            mint.mass_update(deployer) 
        except:
            continue
              

        prices_data.append(
            {
                "on_price": new_price * (1 + deviation_percent / 100),
                "off_price": new_price
            }
        )
        new_price = new_price * (1 + deviation_percent / 100)
        time.sleep(3)

    else:   
        new_price = prices[i] * multiplier;  
        oracle.set_price(deployer, SCA_CONTRACT_ADDR, str(new_price))
        mint.mass_update(deployer) 

        i+= 1
        if i == len(prices):
            i = 0

        time.sleep(5)
    break


df = pd.DataFrame(prices_data)

df.to_csv("data/worker_price.csv", index = False)