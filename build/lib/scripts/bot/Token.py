from re import L
from .Bot import Bot


class Token(Bot):
    ## for simplicity, decimal = 6
    def __init__(self, network_type, deployer_key, symbol, initial_balances, minter) -> None:
        super().__init__(network_type, deployer_key)
        self.token_code_id = self.store_contract("terraswap_token")
        self.symbol = symbol

        initial_balances_data = []
        for user in initial_balances:
            initial_balances_data.append(
                {
                    "address": user[0],
                    "amount": user[1]
                }
            )

        self.contract_addr = self.instantiate_contract(self.token_code_id, {
            "name": symbol,
            "symbol": symbol,
            "decimals": 6,
            "initial_balances": initial_balances_data,
            "mint": {
                "minter": minter
            }
        })

    
    def increase_allowance(self, owner, spender, amount): 
        self.execute_contract(
            owner,
            self.contract_addr,
            {
                "increase_allowance": {
                    "spender": spender,
                    "amount": amount
                }
            },
            additional_msg = self.symbol
        )

    
    def get_balance(self, user):
        return self.query_contract(
            self.contract_addr,
            {
                "balance": {
                    "address": user
                }
            },
            additional_msg= self.symbol
        )

    def __repr__(self) -> str:
        return self.contract_addr