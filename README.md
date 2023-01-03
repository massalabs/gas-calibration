# gas-calibration
A tool to calibrate the gas of the SC

## Context

In Massa, gas is used to limit the usage of the nodes when executing all the smart contracts included in a block. We setup gas costs for each instruction made into the smart contracts and we also set a limit of maximum gas usable in a block and so every block has a limited compute power usage.

## Problem: How to setup gas costs?

This project has the ambition to answer this question by providing a mathematical model to setup gas costs.
For each WASM instruction we want to measure the time it takes to execute it and also if this time vary depending on the input size. Then, when we have the time for this instruction we need to measure how much time this instruction can be called in a block execution to reach the gas limit and so we determine the gas cost for this instruction.

## Explanation of the process

### 1. Generate SCs

The program will generate, for each instruction, a set of smart contracts that will make a random number of calls to this instruction. The number of calls is chosen randomly between 1 and 300.

### 2. Measure the time

The program will run multiple batches of smart contract with the same instruction and for each batch it will measure the time it takes to execute and the number of instructions executed. Using a linear regression we will be able to determine the time it takes to execute one instruction.

### 3. Calculate the gas cost

We have the time it takes to execute one instruction and we also have the gas limit for a block. We can then calculate the number of instruction that can be executed in a block and so the gas cost for this instruction.


## How to use

```bash
cargo run
```

If you already have launched the program once, you can use the following command to only execute the smart contracts and use the generated from the previous run:

```bash
cargo run -- --skip-generation-scs
```

Results are stored in the `results` folder.