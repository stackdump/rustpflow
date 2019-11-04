# rustpflow

Go library to use PetriNets encoded using pflow schema to construct state-machines.

# Status

Work-in-Progres - aiming do demonstrate a state machine in WASM

Will eventually support code generation from a pflow-encoded petri-net.

This was previously achieved in Golang: https://github.com/stackdump/gopflow

# Motivation

Eventually this lib will be used for Factom Asset Token WASM smart contracts.
* https://factomize.com/forums/threads/dbgrow-factom-001-fat-smart-contracts.2029/
* https://github.com/Factom-Asset-Tokens/wasm-contract-poc

Since rust has good support for WASM - it seems like a good tool for constructing a library such as this.

