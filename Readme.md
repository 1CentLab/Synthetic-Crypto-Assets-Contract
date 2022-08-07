## Introduction 
This SCA - Synthetic Crypto Assets is a project that aim to reflex the value of real world asset into the SCA tokens. For example, anyone can create new sGOLD token on the blockchain using this model and allow anyone and everyone to trade sGold. 

## Design 
The model follow the leverage loan design -- The user need to over-collateralize in order to issue new SCA tokens 


## Project architecture 

### contracts  + packages 
Implementation of the model using the CosmWasm framework 

#### Compiling 
./build.sh

### Scripts
Scripts for deployment and interaction on the Terra blockchain. 

#### Running scripts 
For example, deploy all the contract: python3 -m scripts.deploy.all
