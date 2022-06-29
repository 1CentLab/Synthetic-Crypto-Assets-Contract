from .Bot import Bot


class Controller(Bot):
    ## for simplicity, decimal = 6
    def __init__(self, network_type, deployer_key) -> None:
        super().__init__(network_type, deployer_key)
        self.token_code_id = self.store_contract("controller")

        self.contract_addr = self.instantiate_contract(self.token_code_id, {
        })
        self.phrase = "CONTROLLER"


    def add_asset(self,sender, asset): 
        self.execute_contract(
            sender,
            self.contract_addr,
            {
                "add_asset": {
                    "asset": asset
                }
            },
        )

    ### QUERY ### 
    def get_asset_state(self, sca, collateral):
        self.query_contract(
            self.contract_addr,
            {
                "get_asset_state": {
                    "sca": sca,
                    "collateral": collateral
                }
            },
            additional_msg=self.phrase
        )

    def test(self):
        self.query_contract(
            self.contract_addr,
            {
                "test": {
                
                }
            }
        )
    
    def get_sca_oracle_price(self, sca, collateral):
        self.query_contract(
            self.contract_addr,
            {
                "get_sca_oracle_price":{
                    "sca":sca,
                    "collateral": collateral
                }
            },
            self.phrase
        )
    
    def get_sca_pool_reserve(self, sca, collateral):
        self.query_contract(
            self.contract_addr,
            {
                "get_sca_pool_reserve":{
                    "sca":sca,
                    "collateral": collateral
                }
            },
            self.phrase
        )

    ### EXECUTE #### 
    
    def __repr__(self) -> str:
        return self.contract_addr