from dotenv import load_dotenv
from ..bot.Token import Token
from ..bot.Bot import Bot
import os, sys
import json 

load_dotenv()

network = "testnet"

deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)

filepath = os.path.abspath("scripts/data/deployed.json")
deployed_data = json.load(open(filepath))


deployer = bot.get_deployer()
usd = Token(network, deployer_key, "USD", [(deployer.key.acc_address, "1000000000")], deployer.key.acc_address)
sca = Token(network, deployer_key, "SCA", [(deployer.key.acc_address, "1000000000")], deployer.key.acc_address)

deployed_data["usd"] = repr(usd)
deployed_data["sca"] = repr(sca)

with open(filepath, "w") as outfile:
    json.dump(deployed_data, outfile)