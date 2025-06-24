# V0 Linear emissions

The emission distribution code present on Torus V0 is written from scratch while keeping the same behavior as older projects.

The goal here is: on every _epoch_, given a list of consensus members, distribute all _pending emission_ to validators and miners. The former are agents who periodically attest the quality of the miners, and set weights on each accordingly, while the latter produces value through an exposed API.

## Accumulating pending emission

On every new block (target: 8 seconds), a certain amount of tokens is added to the `PendingEmission` storage value, in a way that 64000 tokens are emitted per day (10800 blocks). The amount is controlled by a parameter called `BlockEmission`, which currently is `64000 / 10800`.

Another two factors when calculating the block emission to have in mind are max supply and emission recycling percentage. Those are taken into account inside the `get_total_emission_per_block` function.

## Distribution

Pending emissions are accumulated throughout an epoch, a period in blocks which, once over, is marked by the issuance and distribution of pending tokens. The interval is controlled by the `RewardInterval` storage value, subject to change via proposals, but default value is `100 blocks`.

We use `pallet-balances`' `Currency` API to issue tokens and deposit them afterwards. This is very important. The `issue` functions returns a `NegativeImbalance` which wraps the balance type (u128) and makes it impossible to wrongly distribute more balance than the total pending amount. A mutable reference is passed to the `linear_rewards` function from which the balances are extracted, and, at the end, we set the remaining balance to `PendingEmission` again as a safeguard to avoid burning tokens.

### Consensus members

Agents come and go, sometimes in the middle of an epoch. We must store enough information to distribute emissions even to the agents who were deregistered in the last epoch. This is done through the `ConsensusMembers` storage value, which holds information on weights, last incentives and dividends (used to deregister agents when the network is full).

To avoid having the confusing `uid` from previous implementations, everything is done using account IDs, which may increase storage use, but makes the code a lot more resilient.

Members are (de)registered in a lazy manner. The only time at which the list of members is updated is during distribution.

#### Aggregation

The entire distribution process uses what we call `ConsensusMemberInput` (or only _inputs_) that are gathered in the following order:

1. iterate through all agents delegating weight control to other validators and remove their IDs from the agent list/consensus member list. The input is created by copying the weights of the delegated validator;
2. iterate through all registered agents removing from the consensus member list. If present, an input with the correct weights is created, otherwise, an empty input is used with no weights;
3. finally, iterates through all remaining items in the consensus member list. At this time, the only remaining elements are deregistered agents from the last epoch.

Input creation first calculates the total staked balance and applies the weight penalty factor by doing `stake * (1 - weight_penalty)`. If the total stake is larger than the `MinValidatorStake` value and it has weights set, the weights are filtered so that weights set on themselves are excluded and the remaining ones only refer to registered agents/agents deregistered on last epoch and are finally normalized.

The final step of the aggregation is defining who gets validator permits. The only agents that can act as validators are those with weights set and the top `MaxAllowedValidators` ordered by stake. The others will have their weights zeroed for the current epoch.

### The linear distribution

After inputs are gathered, the linear algorithm proceeds with these steps:

1. We calculate the miner ranks by applying a sparse matrix multiplication of the normalized weights and validator stakes. This efficiently computes how much each miner should receive based on both their reputation (weights assigned by validators) and the economic stake backing those validators. The resulting ranks are normalized and used as the miner incentives.

2. For validator dividends, we calculate the "bonds" by applying a Hadamard product (element-wise multiplication) between weights and stakes. The Hadamard product is used here to represent the strength of connection between validators and miners - it captures not just that a validator vouches for a miner, but also how significant that validator's economic stake is. This approach ensures that validators with higher stakes have proportionally more influence on the distribution.

3. The bonds are then column-normalized to ensure fair distribution across validators, regardless of the number of miners they support.

4. We calculate the final validator dividends by performing a transpose matrix multiplication between the normalized bonds and the miner incentives. This creates a feedback loop where validators who support successful miners (those receiving higher incentives) are rewarded more.

5. Both incentives and dividends are normalized to prepare for the actual token distribution.

6. The emissions are calculated based on the current incentives ratio, which controls the balance between miner incentives and validator dividends.

7. Weight control fees are processed for delegated validators, where a portion of dividends is transferred to the delegator's validator.

8. The incentives and dividends are "upscaled" to u16 values for compact storage in the consensus members record.

9. Finally, the emissions are distributed to the respective accounts, with additional processing:
   - Dividends are split among stakers based on their stake ratio
   - Delegation fees are applied to the dividends
   - Any remaining emissions are added to the agent's stake

### Weight Control Fee

The weight control fee is a mechanism that enables validators to delegate their weight assignment responsibility to other validators. When an agent delegates their weight control, they must pay a fee to the delegated validator. This fee is calculated as a percentage of their dividend emissions and is transferred during the distribution process.

### Delegation Fee

The staking (delegation) fee is a percentage that validators can set to charge stakers who delegate tokens to them. During distribution, this fee is deducted from the dividend emissions before they are distributed to the stakers. This incentivizes validators to maintain good performance to attract delegations.
