      
from turtle import pos
from ..bot.Bot import Bot
from ..bot.Oracle import Oracle
from ..bot.Token import Token
from ..bot.Pair import Pair
from ..bot.Mint import Mint
from ..bot.Controller import Controller
from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys
import pandas as pd 

load_dotenv() 
network = "localterra"

deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()
user2 = bot.get_lt_wallet("test2")


######## WORKING FLOW ##############
print("\n============> INIT CONTROLLER  =================>")
controller = Controller(network, deployer_key)


print("\n============> INIT MINT  =================>")
mint = Mint(network, deployer_key, repr(controller))

print("\n============> INIT TRADING TOKEN  =================>")
sca = Token(network, deployer_key, "GOLD", [], repr(mint))
usd = Token(network, deployer_key, "USD", [(deployer.key.acc_address, "100000000"),(user2.key.acc_address, "100000000")], deployer.key.acc_address)

print("\n============> INIT ORACLE CONTRACT  =================>")
oracle = Oracle(network, deployer_key, "1000000")
oracle.set_price(deployer, repr(sca), "1000000")

print("\n============> INIT PAIR =================>")
pair = Pair(network, deployer_key, repr(sca), repr(usd), "50")
llp = Token(network, deployer_key, "LLP", [], repr(pair))
pair.set_lp_token(repr(llp))


print("\n ============> SET NEW ASSSET FOR CONTROLLER =================>")
asset = {
    "oracle": repr(oracle),
    "pair": repr(pair),
    "sca": repr(sca),
    "collateral": repr(usd),
    "mcr": "1500000",
    "multiplier": "1000000",
    "premium_rate": "1000000"
}
controller.add_asset(deployer, asset)

print("\n============> SETTING ASSET MINTERS =================>")
mint.set_asset(deployer, asset) # mcr: 150%:  1 gold (10$) => cap collateral: 15$


print("\n============> DEPLOYER MINT NEW GOLD  =================>")
usd.increase_allowance(deployer, repr(mint), "3000000")


prices = [165, 172, 163, 185, 190]
multiplier = 1000000

cprice = prices[0] * multiplier;  
oracle.set_price(deployer, repr(sca), str(cprice))

mcr = 1.5

mint.open_position(deployer, "150000", str(int(mcr * multiplier)))  ## open 1000$ position, ratio: 150%. Collateral amount / ratio / oracle_price  (ratio >= 150%)
position = mint.get_position(deployer.key.acc_address)


general_data = []
data = {
    "sca_price": prices[0],
    "debt": position["debt"],
    "collateral": position["size"],
    "collateral_ratio": int(position["size"]) /(int(position["debt"])   * prices[0])
}
general_data.append(data)

flag = True
for price in prices:
    # skip the fistr price 
    if flag:
        flag = False
        continue 

    cprice = price * multiplier;  
    oracle.set_price(deployer, repr(sca), str(cprice))
    mint.mass_update(deployer)

    position = mint.get_position(deployer.key.acc_address)

    data = {
        "sca_price": price,
        "debt": position["debt"],
        "collateral": position["size"],
        "collateral_ratio": int(position["size"]) / (int(position["debt"]) * price)
    }
    general_data.append(data) 

print(general_data)

df = pd.DataFrame(general_data)

df.to_csv("data/liquidation.csv", index = False)

print(df)
