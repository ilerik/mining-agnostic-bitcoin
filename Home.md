# Mining agnostic bitcoin (BIP Draft)

## Problem

This document will address the issue of a continuous DDoS attack targeting the Bitcoin network, e.g. full nodes mempools constantly being overflowed with transactions carrying small value reduce system primary ability to transfer value (and hence making it perfect store of value). Valid transactions are cheap to create (in the sense of computational effort required) and no adequate mechanism exist to make transaction total value increase probably of its confirmation by the network.

Currently, miners decide which transactions to include in blocks because it's them who are securing Bitcoin network providing proof-of-work certificates stored inside every block header. Miners have to store the whole blockchain at all times, so one of the costs is storage which grows linearly with the transaction size (blockchain size as well). 
The only incentive a person who wants to transfer his bitcoins is allowed to use is setting of transaction fee, that is going directly to the miner. This solution probably was intended to utilize free market to determine appropriate fees, but that is obviously not the case in the current bitcoin network. This fee market is far from free market and is susceptible to the DDoS attack of a kind.

## Solution



## Conclusions
