# rustpflow

[![Build Status](https://travis-ci.org/stackdump/rustpflow.svg?branch=master)](https://travis-ci.org/stackdump/rustpflow)

Go library to use PetriNets encoded using pflow schema to construct state-machines.

# Status

Work-in-Progres - aiming do demonstrate a state machine in WASM

Will eventually support code generation from a pflow-encoded petri-net.

This was previously achieved in Golang: https://github.com/stackdump/gopflow

# Motivation

Eventually this lib will be used for Factom Asset Token WASM smart contracts.
* https://factomize.com/forums/threads/dbgrow-factom-001-fat-smart-contracts.2029/
* https://github.com/Factom-Asset-Tokens/wasm-contract-poc

Since rust has good tooling support for WASM - it seems like a good tool for constructing a library such as this.

## Petri-Net GUI editor

Use the Java app

```
# download the jar
wget https://github.com/FactomProject/ptnet-eventstore/raw/master/pneditor-0.71.jar

# run w/ sun java
/opt/jre1.8.0_211/bin/java -jar /opt/pflow/pneditor-0.71.jar &
```

## Build

Use python to generate rust from pflow xml.

Generate wasm from rust.

```
pip install .
./gen.py
./build.sh
```
