from .Bot import Bot


class Controller(Bot):
    ## for simplicity, decimal = 6
    def __init__(self, network_type, deployer_key) -> None:
        super().__init__(network_type, deployer_key)
        self.token_code_id = self.store_contract("controller")

        self.contract_addr = self.instantiate_contract(self.token_code_id, {
        })
        self.phrase = "CONTROLLER"


    ### QUERY ### 
    def get_state(self):
        self.query_contract(
            self.contract_addr,
            {
                "get_state": {}
            }
        )

    def test(self):
        self.query_contract(
            self.contract_addr,
            {
                "test": {
                
                }
            }
        )

    ### EXECUTE #### 
    
    def __repr__(self) -> str:
        return self.contract_addr