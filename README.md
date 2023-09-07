<h1 align="center">Generate EVM Wallet </h1>
<p align="left">
</p>

## Overview
Generation of EVM wallet with specified parameters.

**Instruction:**
 - "Enter the desired sequence after '0x': " - Here you can enter any sequence of characters that will be searched, for example 0000, 1234, 77777, etc.
 - "Enter the number of wallets to generate: " - The number of wallets we're looking for.

## Parmameters that can be changed for your customization:

**Number of threads**
```rust
const N_THREADS: usize = 8;
```
- _By changing these parameters, you can add saving the addresses that will be encountered when generating the searched wallets with a specific character set._

**Number of repeated characters at the beginning of the address after 0x...**
```rust
const INITIAL_CHARACTERS: usize = 8;
```

**Number of repeating characters in the address anywhere e.g. 0x...0000...**
```rust
const NUMBER_OF_REPEATED_CHARACTERS: usize = 8;
```

## Installing Rust and Cargo.

Install the required software from the link: https://doc.rust-lang.org/cargo/getting-started/installation.html

## Launch.

```
cargo run --release
```

## Donation:
```bash
0x51b7f952bf7b34f82e083067ef06dfd7f70eb381
```
- _only EVM_
