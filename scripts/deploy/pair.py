from asyncio import constants
from ..bot.Bot import Bot
from ..bot.Pair import Pair
from ..bot.Token import Token
from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os, sys, json, base64

load_dotenv() 

filepath = os.path.abspath("scripts/data/deployed.json")
deployed_data = json.load(open(filepath))

network = "testnet"

deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()

try:
    usd_addr = deployed_data["usd"]
    sca_addr = deployed_data["sca"]
except:
    print("Please setting token first")
    sys.exit()

try: 
    pair_addr = deployed_data["pair"]
    lp_addr = deployed_data["llp"]
    pair = Pair(network, deployer_key, "", "", "30", pair_addr)
    lp = Token(network, deployer_key, "LLP", [(deployer.key.acc_address, "100")], repr(pair), lp_addr)

except:
    pair = Pair(network, deployer_key, sca_addr, usd_addr, "30")
    lp = Token(network, deployer_key, "LLP", [(deployer.key.acc_address, "100")], repr(pair))

    deployed_data["pair"] = repr(pair)
    deployed_data["llp"] = repr(lp)

pair.set_lp_token(repr(lp))

with open(filepath, "w") as outfile:
    json.dump(deployed_data, outfile)