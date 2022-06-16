from terra_sdk.client.localterra import LocalTerra

# Create client to communicate with localterra.
terra = LocalTerra()

# Initialize preconfigured test wallet.
wallet = terra.wallets["test1"]

print(wallet.key.acc_address)