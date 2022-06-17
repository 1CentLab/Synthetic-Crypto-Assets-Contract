## Introduction 
This SCA - Synthetic Crypto Assets is a project that aim to mirror the price of real world asset into the price of the SCA tokens 

+ stable model
 - Real world 1:1 backing : USDT 
 - Algo stable model: Terra
 - Collateral model: DAI, SCA 

+ Weakness and strength 
 

## Motivation 
- Tokenize 
  + For example, in some countries, it's not easy to have access to some common assets like golds, oils, or stocks of big company. Blockchain and SCA will remove that limitation

- Private Accesss Model *** 
    + Can be used as private model for company who wants to distribute ownership of some specific assets. To do this, we may need KYC to identify who is possess what, and to insure the liability of the system 

- Fully ownership
    + We are aiming to make a model that mapping the token to actual assets. For example, if user holding 1 sAAPL stock on our system, that token should the same value as the true stock in real world. By having a third party that have the legal right to do that, we can modify the model and allow a broader access to the assets that we're aiming to 


##  Project structure 
 - USD - SCA tokens: Collateral token and SCA token
 - Pair: Mock pool (has similar functionality as uniswap)
 - Oracle: Contract that handles and feed real world sca price 
 - Mint: Contract that handle the process of minting new SCA, liquidate collateral 
 - Controller: contract gather collateral from pool, liquidated events  and perform auctions... 

 ## Folder structure 
 - Contracts + packages: Smart contract and its packages 
 - Scripts: Deploy + test scripts
 - Scripts/ data /deployed: Data deployed 

## Todo 
https://docs.terra.money/docs/develop/terra-js/smart-contracts.html
- Recheck the pair logic  + withdrawal 


