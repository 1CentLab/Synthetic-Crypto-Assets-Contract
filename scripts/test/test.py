
import pandas as pd 

data = [{'sca_price': 165, 'debt': '606', 'collateral': '150000', 'collateral_ratio': 0.0}, {'sca_price': 172, 'debt': '533', 'collateral': '137304', 'collateral_ratio': 44308.232645403375}, {'sca_price': 170, 'debt': '533', 'collateral': '137304', 'collateral_ratio': 43793.020637898684}, {'sca_price': 185, 'debt': '419', 'collateral': '116098', 'collateral_ratio': 51260.45346062053}, {'sca_price': 190, 'debt': '385', 'collateral': '109464', 'collateral_ratio': 54021.194805194806}]


df = pd.DataFrame(data)

print(df)