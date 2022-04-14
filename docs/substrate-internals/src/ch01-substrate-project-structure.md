# Substrate 项目结构

一个标准的 Substrate 由几个核心组件组成。

## Node

区块链节点是一个允许用户参与区块链网络的应用程序。基于 Substrate 的区块链节点暴露了一些能力：

* 网络: Substrate 节点使用 `libp2p` 网络协议栈，使网络中的节点能够互相通信。
* 共识：区块链必须有一种方式来达成对网络状态的共识。Substrate 使得提供定制的共识引擎成为可能，并且还提供了几个建立在 Web3 基金会研究之上的共识机制。
* RPC 服务：一个 RPC 服务器，用来和 Substrate 节点进行交互。

`node` 目录中有几个核心文件：

* `chain_spec.rs`: 即 chain specification，定义了一条 Substrate 链的初始 (genesis) 状态。chain specification 对开发和测试很有用，在架构一条生产环境的链的启动时也很关键。请注意 `development_config` 和 `testnet_genesis` 这两个函数，它们被用来定义本地开发链配置的创世状态。这些函数识别一些知名账户，并使用它们来配置区块链的初始状态。
* `service.rs`: 这个文件定义了节点的实现。注意这个文件导入的库和它调用的函数的名称。特别是，这里提到了与共识相关的主题，如最：长链规则、Aura 区块编写机制和 GRANDPA 最终一致性程序。

在节点编译完成后，可以通过命令行参数，了解更多他所暴露的能力和配置参数。

```sh
./target/release/template-node --help
```

## Runtime

在 Substrate 中，术语 "runtime(运行时)"和 "state transition function(状态转换函数)"是类似的 -- 它们指的是区块链的核心逻辑，负责验证区块并执行它们定义的状态变化。Substrate 项目使用 FRAME 框架来构建区块链运行时。FRAME 允许运行时开发人员在称为 `pallet` 的模块中声明特定领域的逻辑。FRAME 的核心是一种有用的宏语言，它使创建 `pallet` 变得容易，并灵活地组合它们，以创建可以解决各种需求的区块链。

`runtime` 有几个核心点：

* 每个 `runtime` 配置了多个 `pallet`, 来包含到运行时中。每个 `pallet` 配置由一个代码块定义，该代码块以 `impl $PALLET_NAME::Config for Runtime` 开头。
* 不同的 `pallet` 通过 `construct_runtime!` 宏来组织到一个 `runtime` 中。

## Pallet

Substrate 项目的 `runtime` 是由许多 FRAME pallet 组成的。

一个 FRAME pallet 由许多区块链原语 (primitive) 组成：

* Storage: 存储，FRAME 框架定义了一套丰富的强大的存储抽象，使得使用 Substrate 的高效键值数据库来管理区块链的不断变化的状态变得容易。
* Dispatchable: 可调用方法，FRAME pallet 定义了特殊类型的函数，可以从运行时外部调用 (dispatched)，以更新其状态。
* Event: 事件，Substrate 使用事件来通知用户在运行时中的重要变化。
* Errors: 错误，当一个 dispatchable 函数失败时，它会返回一个错误。
* Config: 配置，配置接口用于定义 FRAME pallet 所依赖的类型和参数。
