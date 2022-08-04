from .Bot import Bot


class Mint(Bot):
    ## for simplicity, decimal = 6
    def __init__(self, network_type, deployer_key, controller, contract_addr=None) -> None:
        super().__init__(network_type, deployer_key)
        if contract_addr == None:
            self.token_code_id = self.store_contract("mint")

            self.contract_addr = self.instantiate_contract(self.token_code_id, {
                "controller": controller
            })
        else: 
            print(f"** Getting contract at: {contract_addr}")
            self.contract_addr = contract_addr





    def get_sca_oracle_price(self):
        self.query_contract(
            self.contract_addr,
            {
                "get_sca_oracle_price": {}
            },
            additional_msg="ORACLE_PRICE"
        )

    def get_position(self, user):
        return self.query_contract(
            self.contract_addr,
            {
                "get_position": {
                    "user": user
                }
            },
            additional_msg="POSITION"
        )

    def get_all_positions(self):
        self.query_contract(
            self.contract_addr,
            {
                "get_all_positions": {
    
                }
            },
            additional_msg="ALL_POSITION"
        )

    
    def get_sca_pool_reserves(self):
        self.query_contract(
            self.contract_addr,
            {
                "get_sca_pool_reserve": {

                }
            },
            additional_msg="POOL_RESERVE"
        )


    ### SETTER ######333
    def set_asset(self, sender, asset):
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

    def mass_update(self, sender):
        self.execute_contract(
            sender,
            self.contract_addr,
            {
                "mass_update":{}
            }
        )

    def __repr__(self) -> str:
        return self.contract_addr