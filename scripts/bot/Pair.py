from .Bot import Bot



class Pair(Bot):
    def __init__(self, network_type, deployer_key, token0, token1,fee, contract_addr=None) -> None:
        super().__init__(network_type, deployer_key)

        if contract_addr == None:
            self.code_id = self.store_contract("pair")
            self.contract_addr = self.instantiate_contract( 
                self.code_id,
                {
                    "token0": token0,
                    "token1": token1,
                    "fee": fee
                }
            )
        else:
            print(f"** Getting contract at: {contract_addr}")
            self.contract_addr = contract_addr
    

    def get_lp_token_info(self, user):
        balance = self.query_contract(self.contract_addr, {
            "get_lp_token_info": {
                "user": user
            },
        }
        , "GET_LP_TOKEN_INFO")
        return balance

    def get_reserves(self):
        return self.query_contract(
            self.contract_addr,
            {
                "get_reserves": {}
            },
            "GET_RESERVES"
        )


    def get_amounts_out(self, amount_in, path):
        return self.query_contract(
            self.contract_addr,
            {
                "get_amounts_out": {
                    "amount_in": amount_in,
                    "path": path
                }
            },
            "GET_AMOUNTS_OUT"
        )

    ###### EXECUTE FUNCTION #################33333
    
    def set_lp_token(self, lp_token):
        self.execute_contract(self.deployer, self.contract_addr, {
            "set_lp_token": 
             {
                 "lp_token": lp_token
             }
        })
    
    def add_liquid(self, caller, amount0, amount1):
        self.execute_contract(caller, self.contract_addr, {
            "add_liquid": {
                "amount0": amount0,
                "amount1": amount1
            }
        })
    
    def migrate(self, new_code_id, migrate_msg):
        self.migrate_contract(self.contract_addr, new_code_id, migrate_msg)

    

    def remove_liquid(self, caller, liquid):
        self.execute_contract(caller, self.contract_addr, {
            "remove_liquid": {
                "liquid": liquid,
            }
        })

    
    def swap(self,caller, amount_in, path):
        self.execute_contract(
            caller,
            self.contract_addr,
            {
                "swap":{
                    "amount_in": amount_in,
                    "path": path
                }
            }
        )
    

    def __repr__(self) -> str:
        return self.contract_addr