# Decentrilized proof-of-work and DDoS resistance for Bitcoin

## Abstract

By introducing some new block validation rules it is possible to make Bitcoin network more secure, decentrilized and DDoS resistant. The idea is to modify simple proof-of-work puzzle in such a way that user transactions could be hardened with the same proof-of-work algorithm thus insentivising all the miners to include that particular transaction. Such mechanism will effectively give handicap to every miner who includes "mined" transaction into next block, increasing probability of getting block reward.

## Problems and motivation

This document will address the issue of a continuous DDoS attack targeting the Bitcoin network, e.g. full nodes mempools constantly being overflowed with transactions carrying small value reduce system primary ability to transfer value (and hence making it perfect store of value). Valid transactions are cheap to create (in the sense of computational effort required) and no adequate mechanism exist to make transaction total value increase probably of its confirmation by the network.

Currently, miners decide which transactions to include in blocks because it's them who are securing Bitcoin network providing proof-of-work certificates stored inside every block header. Miners have to store the whole blockchain at all times, so one of the costs is storage which grows linearly with the transaction size (blockchain size as well). Other cost is network bandwidth which depends directly on the size of data to be communicated over.

The only incentive a person who wants to transfer his bitcoins is allowed to use is setting of transaction fee, that is going directly to the miner. This solution probably was intended to utilize free market (as implied by Satoshi introducing sequence numbers) to determine appropriate fees, but that is obviously not the case, in the current bitcoin network operating in full block capacity mode. This fee market deviates significantly from free market premise (also attempts being made to make it closer, e.g. in [BIP125](https://github.com/bitcoin/bips/blob/master/bip-0125.mediawiki) where Replace-By-Fee signaling is suppose to help in replacing "stuck" transactions with noncompetitive fee).

Currently bitcoin network is susceptible to the DDoS attack of a kind. Adversary creating and translating into the network a lot of transactions carrying small value (e.g. only miners fee), will be able to impair ability to transfer value for everyone in the world, should he has enough money to pay the fees. Miners would continue to work providing security for the network and new blocks will consist of transaction transfering negligible value. It's major drawback and maleability, because cost of such attack doesn't grow asymmetrically with the cost of BTC asset.

## Proposed solution

So how do we incentivize all miners to include our transaction carrying a lot of value in the next block? The only thing miner supposed to do to get reward is to produce Hashcash proof-of-work, thus providing security guaranties for the whole Bitcoin blockchain. What if including our transaction in block would reduce effort requirements for miner generating hash?

We could do so by extending concept of proof-of-work, in such a way that we do not sacrifice security at all. Here is both descriptons proof-of-work as-is and to-be:

1. Standart proof-of-work: 
hash(previous block hash + current block target + current block metadata + current block transactions) < target

2. Decentrilized proof-of-work:
hash(previous block hash + current block target + current block metadata + current block transactions) - sum( FFFF - hash( previous block hash + raw_tx ) ) < target

It is possible to imagine completely mining agnostic proof-of-work, for example the following PoW would do:

3. Distributed (mining-agnostic) proof-of-work:
sum( FFFF - hash( previous block hash + current block target + current block metadata + signed_tx ) ) < target

## Economical reasoning

Adversary whos goal is to prevent network from transfering value will have to compete with good users hashrate using same equipment good miners will use. And it's far more complicated then competing with others using money to pay  transaction fees.

In order to investigate probable consequences of protocol upgrade and stability of implied economical equilibrium, we need adequate game theoretical model. Such model and numerical simulation results should be obtained and studied before any protocol change could be considered by community.

## Conclusions

Described protocol change could be implemented in two ways:
1. (Preferred) User activated soft-fork (described in [BIP148](https://github.com/bitcoin/bips/blob/master/bip-0148.mediawiki)), introducing new blocks with modified proof-of-work concept.
2. Hardfork updating miners software and it's block validation rules directly.

To me it seems like win-win solution for everyone owning BTC:
1. Miners benefit: as the result DDoS attack will be stopped, Bitcoin becomes perfect store-of-value finally. Political decentralization of hashrate will be incentivized as mining equipment becomes relevant to every user.
2. Users benefit: miners will have direct incentives to include transactions deemed important by their sender and supported by some amount of proof-of-work. 