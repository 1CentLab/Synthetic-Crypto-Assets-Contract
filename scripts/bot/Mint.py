from bot.Bot import Bot


class Mint(Bot):
    ## for simplicity, decimal = 6
    def __init__(self, network_type, deployer_key) -> None:
        super().__init__(network_type, deployer_key)
        self.token_code_id = self.store_contract("mint")

        self.contract_addr = self.instantiate_contract(self.token_code_id, {
        })





    def get_sca_oracle_price(self):
        self.query_contract(
            self.contract_addr,
            {
                "get_sca_oracle_price": {}
            }
        )

    def get_position(self, user):
        self.query_contract(
            self.contract_addr,
            {
                "get_position": {
                    "user": user
                }
            }
        )

    def get_all_positions(self):
        self.query_contract(
            self.contract_addr,
            {
                "get_all_positions": {
    
                }
            }
        )

    
    def get_sca_pool_price(self):
        self.query_contract(
            self.contract_addr,
            {
                "get_sca_pool_reserve": {

                }
            }
        )


    ### SETTER ######333
    def set_asset(self, sender, oracle, pair, sca, collateral, mcr, multiplier):
        asset = {
            "oracle": oracle,
            "pair": pair,
            "sca": sca,
            "collateral": collateral,
            "mcr": mcr, 
            "multiplier": multiplier
        } 

        self.execute_contract(
            sender,
            self.contract_addr,
            {
                "set_asset":{
                    "asset": asset
                }
            }
        )

    
    def open_position(self, sender, collateral_amount, ratio):
        self.execute_contract(
            sender, 
            self.contract_addr,
            {
                "open_position": {
                    "collateral_amount": collateral_amount,
                    "ratio": ratio
                }
            }
        )

    def close_position(self, sender, sca_amount):
        self.execute_contract(
            sender,
            self.contract_addr,
            {
                "close_position":{
                    "sca_amount": sca_amount
                }
            }
        )

    def __repr__(self) -> str:
        return self.contract_addr