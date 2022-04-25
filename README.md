Hello Substrate
---

Reference : https://docs.substrate.io/tutorials/v3/ink-workshop/pt1/

## Testing

```
cargo +nighty test
```

## Build a contract

```
cargo +nightly contract build
```

> `cargo-contract` cannot build using "stable" channel. ref: https://github.com/paritytech/cargo-contract#build-requires-the-nightly-toolchain


Result similar to this:

```
Original wasm size: 48.4K, Optimized: 22.4K

The contract was built in DEBUG mode.

Your contract artifacts are ready. You can find them in:
/flipper/target/ink

  - flipper.contract (code + metadata)
  - flipper.wasm (the contract's code)
  - metadata.json (the contract's metadata)
```

Start Substrate node

```
substrate-contracts-node --dev
```

Deploy contract with [contract-ui](https://paritytech.github.io/contracts-ui/)

## Makefile

```
make test name=flipper
make test name=incrementer
```

## How to use mapping?

1. Add `SpreadAllocate` trait on the storage structure.
2. Specific key, value and dont forget to add derive `#[derive(SpreadAllocate)]`
3. Call `ink_lang::utils::initialize_contract` to initialize mapping.