from .Bot import Bot


class Oracle(Bot):
    ## for simplicity, decimal = 6
    def __init__(self, network_type, deployer_key, multiplier) -> None:
        super().__init__(network_type, deployer_key)
        self.token_code_id = self.store_contract("oracle")

        self.contract_addr = self.instantiate_contract(self.token_code_id, {
            "multiplier": multiplier
        })


    def set_price(self, sender, sca, price):
        self.execute_contract(
            sender,
            self.contract_addr,
            {
                "set_price": {
                    "sca": sca,
                    "price": price
                }
            }
        )
    

    def get_price(self, sca):
        self.query_contract(
            self.contract_addr,
            {
                "get_price": {
                    "sca": sca
                }
            }
        )

    def __repr__(self) -> str:
        return self.contract_addr