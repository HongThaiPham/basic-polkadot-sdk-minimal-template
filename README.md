# Minimal Template

This is a minimal template for creating a blockchain using the Polkadot SDK.

# Docs

You can generate and view the [Rust
Docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for this template
with this command:

```sh
cargo doc -p minimal-template --open
```

# Compile

```sh
cargo c -p pallet-minimal-template
```

# Test

```sh
cargo t -p pallet-minimal-template
```

# Build runtime

```sh
cargo b -p minimal-template-runtime
```

#Run node

```sh
pba-omni-node --runtime ./target/release/wbuild/minimal-template-runtime/minimal_template_runtime.wasm --tmp


```

```sh
chain-spec-builder create --chain-name pba-frame -r ./target/release/wbuild/minimal-template-runtime/minimal_template_runtime.compact.compressed.wasm default

```

## add this to chain-spec.json balances:

```json
"balances": {
          "balances": [
            ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 100000000000],
            ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 100000000000],
            ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 100000000000],
            ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 100000000000],
            ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", 100000000000],
            ["5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL", 100000000000],
            ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", 100000000000],
            ["5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc", 100000000000],
            ["5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT", 100000000000],
            ["5HKPmK9GYtE1PSLsS1qiYU9xQ9Si1NcEhdeCq9sw5bqu4ns8", 100000000000],
            ["5FCfAonRZgTFrTd9HREEyeJjDpT397KMzizE6T3DvebLFE7n", 100000000000],
            ["5CRmqmsiNFExV6VbdmPJViVxrWmkaXXvBrSX8oqBT8R9vmWk", 100000000000],
            ["5Fxune7f71ZbpP2FoY3mhYcmM596Erhv1gRue4nsPwkxMR4n", 100000000000],
            ["5CUjxa4wVKMj3FqKdqAUf7zcEMr4MYAjXeWmUf44B41neLmJ", 100000000000]
          ]
        },
```

```sh
pba-omni-node --chain chain_spec.json --tmp
```
