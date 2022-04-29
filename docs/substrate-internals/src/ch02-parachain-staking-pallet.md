# Parachain Staking 模块分析

项目地址: <https://github.com/bifrost-finance/parachain-staking>

平行链质押模块实现了通过总支持质押量进行收集人 (collator) 选举。这个 pallet 与 `frame/pallet-staking` 的区别是这个 pallet 使用直接委托 (delegation). 委托人 (delegator) 直接选择他们委托给谁以及委托数量，而 `frame/pallet-staking` 里委托人通过授权投票和运行 [Phragmen 投票规则](https://en.wikipedia.org/wiki/Phragmen%27s_voting_rules) 来实现。

## 规则

每 `<Round<T>>::get().length` 个区块会有新一轮 (round) 统计和结算等工作。

在每一轮开始阶段

* 对于收集人 (以及他们的委托人) 的发行量计算，计算 `T::RewardPaymentDelay` 轮之前的区块生成。
* 从候选人 (candidate) 中选出新的收集人集合。

紧接着轮换后，每一区块支付一次，直到所有的支付都完成。在每一个这样的区块中，一个收集人被选中进行奖励支付，并与他的每个前排`T::MaxTopDelegationsPerCandidate`委托人一起被支付。

要加入候选人集合，请调用`join_candidates`，其中`bond >= MinCandidateStk`。
要离开候选人集合，调用`schedule_leave_candidates`。如果调用成功，收集人将从候选池中移除，因此他们不能被选入未来的收集人集合，但在他们的退出请求被执行之前，他们不会被解除绑定。任何已签署的账户都可以在提出原始请求的轮次之后触发退出`T::LeaveCandidatesDelay`轮次。

要加入委托人集合，请调用`delegate`，并传入一个已经是收集人候选人的账户，并且`bond >= MinDelegatorStk`。每个委托人可以通过调用`delegate`委托多达`T::MaxDelegationsPerDelegator`的收集人候选人。

要撤销一个委托，请用收集人候选人的账户调用`revoke_delegation`。

要离开委托人集合并撤销所有委托，请调用`leave_delegators`。

## 撤销委托流程

用户调用 `schedule_revoke_delegation` 接口进行撤销质押。在 `DelegationRequest` 结构中包含了 `when_executable`, 也就是质押失效的轮次信息。

## 奖励发放流程

在 Pallet Hooks 里的 `on_initialize` 阶段进行，本轮奖励计算都在 `prepare_staking_payouts` 方法中，应该是这里没有处理 Revoke 状态的 delegation. Revoke 的 delegation 需要记录在哪一轮撤销的，然后失效后就不再进行计算。所以，需要在这里读取没有失效的 delegation，然后再进行奖励计算。
