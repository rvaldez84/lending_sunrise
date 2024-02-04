## Sunrise Smart Contract

Prebuilt Binaries

Raw, optimized, and meta WASM binaries can be found in the Releases section.


Building Locally


⚙️ Install Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

⚒️ Add specific toolchains

```
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```
... or ...

```
make init
```

🏗️ Build

```
cargo build --release
```
... or ...

```
make build
```

✅ Run tests

```
cargo test --release
```
... or ...

```
make test
```

🚀 Run everything with one command

```
make all
```
... or just ...

```
make
```

License

The source code is licensed under the MIT license.
