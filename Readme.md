


https://docs.terra.money/docs/develop/terra-js/smart-contracts.html



## Introduction 
This SCA - Synthetic Crypto Assets is a project that aim to mirror the price of real world asset into the price of the SCA tokens 

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

- Recheck the pair logic  + withdrawal 
