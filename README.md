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