from .bot.Bot import Bot
from .bot.Oracle import Oracle
from .bot.Token import Token
from terra_sdk.client.lcd import LCDClient
from dotenv import load_dotenv
import os

load_dotenv() 
network = "localterra"

deployer_key = os.environ.get("MNEMONIC_KEY")
bot = Bot(network, deployer_key)
deployer = bot.get_deployer()
user2 = bot.get_lt_wallet("test2")


# init new token contract
gold = Token(network, deployer_key, "GOLD", [(deployer.key.acc_address, "1000"),(user2.key.acc_address, "1000")], deployer.key.acc_address)


# init new oracle contract
oracle = Oracle(network, deployer_key, "1000000")

oracle.set_price(deployer, repr(gold), "20000000")
oracle.get_price(repr(gold))