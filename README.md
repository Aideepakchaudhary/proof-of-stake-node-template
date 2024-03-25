# Substrate Node Template

A fresh [Substrate](https://substrate.io/) node, ready for hacking :rocket:

A standalone version of this template is available for each release of Polkadot in the [Substrate Developer Hub Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template/) repository.
The parachain template is generated directly at each Polkadot release branch from the [Node Template in Substrate](https://github.com/paritytech/substrate/tree/master/bin/node-template) upstream

It is usually best to use the stand-alone version to start a new project.
All bugs, suggestions, and feature requests should be made upstream in the [Substrate](https://github.com/paritytech/substrate/tree/master/bin/node-template) repository.

## Getting Started

Depending on your operating system and Rust version, there might be additional packages required to compile this template.
Check the [Install](https://docs.substrate.io/install/) instructions for your platform for the most common dependencies.
Alternatively, you can use one of the [alternative installation](#alternatives-installations) options.

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Embedded Docs

After you build the project, you can use the following command to explore its parameters and subcommands:

```sh
./target/release/node-template -h
```

You can generate and view the [Rust Docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for this template with this command:

```sh
cargo +nightly doc --open
```

### Single-Node Development Chain

The following command starts a single-node development chain that doesn't persist state:

```sh
./target/release/node-template --dev
```

To purge the development chain's state, run the following command:

```sh
./target/release/node-template purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

Development chains:

- Maintain state in a `tmp` folder while the node is running.
- Use the **Alice** and **Bob** accounts as default validator authorities.
- Use the **Alice** account as the default `sudo` account.
- Are preconfigured with a genesis state (`/node/src/chain_spec.rs`) that includes several prefunded development accounts.


To persist chain state between runs, specify a base path by running a command similar to the following:

```sh
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/node-template --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint.
A hosted version is also available on [IPFS (redirect) here](https://dotapps.io/) or [IPNS (direct) here](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).
You can also find the source code and instructions for hosting your own instance on the [polkadot-js/apps](https://github.com/polkadot-js/apps) repository.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, see [Simulate a network](https://docs.substrate.io/tutorials/build-a-blockchain/simulate-network/).

## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to [consensus](https://docs.substrate.io/fundamentals/consensus/) on the state of the network.
  Substrate makes it possible to supply custom consensus engines and also ships with several consensus mechanisms that have been built on top of [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory.
Take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A [chain specification](https://docs.substrate.io/build/chain-spec/) is a source code file that defines a Substrate chain's initial (genesis) state.
  Chain specifications are useful for development and testing, and critical when architecting the launch of a production chain.
  Take note of the `development_config` and `testnet_genesis` functions,.
  These functions are used to define the genesis state for the local development chain configuration.
  These functions identify some [well-known accounts](https://docs.substrate.io/reference/command-line-tools/subkey/) and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation.
  Take note of the libraries that this file imports and the names of the functions it invokes.
  In particular, there are references to consensus-related topics, such as the [block finalization and forks](https://docs.substrate.io/fundamentals/consensus/#finalization-and-forks) and other [consensus mechanisms](https://docs.substrate.io/fundamentals/consensus/#default-consensus-models) such as Aura for block authoring and GRANDPA for finality.



### Runtime

In Substrate, the terms "runtime" and "state transition function" are analogous.
Both terms refer to the core logic of the blockchain that is responsible for validating blocks and executing the state changes they define.
The Substrate project in this repository uses [FRAME](https://docs.substrate.io/learn/runtime-development/#frame) to construct a blockchain runtime.
FRAME allows runtime developers to declare domain-specific logic in modules called "pallets".
At the heart of FRAME is a helpful [macro language](https://docs.substrate.io/reference/frame-macros/) that makes it easy to create pallets and flexibly compose them to create blockchains that can address [a variety of needs](https://substrate.io/ecosystem/projects/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note the following:

- This file configures several pallets to include in the runtime.
  Each pallet configuration is defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- The pallets are composed into a single runtime by way of the [`construct_runtime!`](https://paritytech.github.io/substrate/master/frame_support/macro.construct_runtime.html) macro, which is part of the [core FRAME pallet library](https://docs.substrate.io/reference/frame-pallets/#system-pallets).

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with [the Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is comprised of a number of blockchain primitives, including:

- Storage: FRAME defines a rich set of powerful [storage abstractions](https://docs.substrate.io/build/runtime-storage/) that makes it easy to use Substrate's efficient key-value database to manage the evolving state of a blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched) from outside of the runtime in order to update its state.
- Events: Substrate uses [events](https://docs.substrate.io/build/events-and-errors/) to notify users of significant state changes.
- Errors: When a dispatchable fails, it returns an error.

Each pallet has its own `Config` trait which serves as a configuration interface to generically define the types and parameters it depends on.

## Alternatives Installations

Instead of installing dependencies and building this source directly, consider the following alternatives.

### Nix

Install [nix](https://nixos.org/) and
[nix-direnv](https://github.com/nix-community/nix-direnv) for a fully plug-and-play
experience for setting up the development environment.
To get all the correct dependencies, activate direnv `direnv allow`.

### Docker

Please follow the [Substrate Docker instructions here](https://github.com/paritytech/substrate/blob/master/docker/README.md) to build the Docker container with the Substrate Node Template binary.


### Keys:
1.
Secret phrase:       lens obscure stool fiber idle toy hat embody air sudden rare parade
Network ID:        substrate
Secret seed:       0xb57dd58fce44be3c99f9cd4fd5cd8bce40a8b8c72df48fbfb0a069f4514d4ec9
Public key (hex):  0x5c4470591d9ada5bc1ef37366a0d2213026e593d61bfa32e55bdd3cc24385f27
Account ID:        0x5c4470591d9ada5bc1ef37366a0d2213026e593d61bfa32e55bdd3cc24385f27
Public key (SS58): 5E9gbMacn5jVDiP5J6159vMNbQspU8ToRo5Uut7VDKFifjJu
SS58 Address:      5E9gbMacn5jVDiP5J6159vMNbQspU8ToRo5Uut7VDKFifjJu

Secret phrase:       lens obscure stool fiber idle toy hat embody air sudden rare parade
Network ID:        substrate
Secret seed:       0xb57dd58fce44be3c99f9cd4fd5cd8bce40a8b8c72df48fbfb0a069f4514d4ec9
Public key (hex):  0x23814a60d1dc9fb7426f2f962027a2e45af9e16bbb0a5a55a494a2ffd5fb61b5
Account ID:        0x23814a60d1dc9fb7426f2f962027a2e45af9e16bbb0a5a55a494a2ffd5fb61b5
Public key (SS58): 5CsFwVtMojE72q7TQA6k5dWAaRmMNx6wDnXHBQN7Zk9zdfV1
SS58 Address:      5CsFwVtMojE72q7TQA6k5dWAaRmMNx6wDnXHBQN7Zk9zdfV1

2.
Secret phrase:       notable pull amount goat prize doctor waste mask diagram bomb example cube
Network ID:        substrate
Secret seed:       0xf90bf2ec059a5045428889e7527b83be03656ceeeaaedd87516a543e1578242e
Public key (hex):  0xf2678a38ef0da47165f7a505048f08cdc10551968c7ce9268eacedb64094ae1b
Account ID:        0xf2678a38ef0da47165f7a505048f08cdc10551968c7ce9268eacedb64094ae1b
Public key (SS58): 5HYYCjHoMvNVRXBfavocxh6Y5VCnms3z1DCE9MGsWE5P7FBy
SS58 Address:      5HYYCjHoMvNVRXBfavocxh6Y5VCnms3z1DCE9MGsWE5P7FBy

Secret phrase:       notable pull amount goat prize doctor waste mask diagram bomb example cube
Network ID:        substrate
Secret seed:       0xf90bf2ec059a5045428889e7527b83be03656ceeeaaedd87516a543e1578242e
Public key (hex):  0x89d05c38f2527bee6d0aa5249b872ca1157447ca59f621d45c4b05d084358ca7
Account ID:        0x89d05c38f2527bee6d0aa5249b872ca1157447ca59f621d45c4b05d084358ca7
Public key (SS58): 5FBQK89XGQ4Ah42YMgXMb7QbTct35wwNzQEHynR8UwgfnFqo
SS58 Address:      5FBQK89XGQ4Ah42YMgXMb7QbTct35wwNzQEHynR8UwgfnFqo

3.
Secret phrase:       core scheme pupil prison light ramp lazy excuse fashion human fresh abandon
Network ID:        substrate
Secret seed:       0xa223dd13595ba5bc102f2b78cd68752528d80e2102932072d19fa218e017954d
Public key (hex):  0x26f1116267439b1b663b6c9b0753bb27c0c42cee08c3b67d846b76c86907257a
Account ID:        0x26f1116267439b1b663b6c9b0753bb27c0c42cee08c3b67d846b76c86907257a
Public key (SS58): 5CwmHi8fz6kcJJTSZVn9125joPY13VyrJ8uP4mvuaXQXuAxY
SS58 Address:      5CwmHi8fz6kcJJTSZVn9125joPY13VyrJ8uP4mvuaXQXuAxY


Secret phrase:       core scheme pupil prison light ramp lazy excuse fashion human fresh abandon
Network ID:        substrate
Secret seed:       0xa223dd13595ba5bc102f2b78cd68752528d80e2102932072d19fa218e017954d
Public key (hex):  0xbe87dc9d96f930ac22a3ce8b9a37f7f35828cffac4c5ad1a4887d350553bacbe
Account ID:        0xbe87dc9d96f930ac22a3ce8b9a37f7f35828cffac4c5ad1a4887d350553bacbe
Public key (SS58): 5GNXKCgKvLYuAzQaKcTTDduPSnn2Lr5Wk3Z8BC5YbRXuPY4S
SS58 Address:      5GNXKCgKvLYuAzQaKcTTDduPSnn2Lr5Wk3Z8BC5YbRXuPY4S


4.
Secret phrase:       banana hello saddle erupt process globe rail helmet hawk plunge cancel fine
Network ID:        substrate
Secret seed:       0x3a5bd5625b0857b61af492efe70e9f424dba1ba1e4ef75d0ccc524b6a8bc7018
Public key (hex):  0x6e7897036744b93b5c5ae1aaaea40d9b57d81a3801e6e511ba0a6daa3de1303e
Account ID:        0x6e7897036744b93b5c5ae1aaaea40d9b57d81a3801e6e511ba0a6daa3de1303e
Public key (SS58): 5EZYwtVqNUjRtgx1mLGJz31q26yWNzWoUySddUF2EgnduHXo
SS58 Address:      5EZYwtVqNUjRtgx1mLGJz31q26yWNzWoUySddUF2EgnduHXo

Secret phrase:       banana hello saddle erupt process globe rail helmet hawk plunge cancel fine
Network ID:        substrate
Secret seed:       0x3a5bd5625b0857b61af492efe70e9f424dba1ba1e4ef75d0ccc524b6a8bc7018
Public key (hex):  0xafcebd8591a8df94bff1702f65239b307d318de2a0b9f66454b633446cbaca9a
Account ID:        0xafcebd8591a8df94bff1702f65239b307d318de2a0b9f66454b633446cbaca9a
Public key (SS58): 5G3DekXYwq4S7zqMuLctV6M48vENn8BRH629W4ZwUa6kXsFX
SS58 Address:      5G3DekXYwq4S7zqMuLctV6M48vENn8BRH629W4ZwUa6kXsFX

