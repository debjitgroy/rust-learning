# hello_cargo

The same hello world, but built with Cargo instead of calling `rustc` directly.

## Build and run

```bash
cargo run
```

Compiles and runs in one step. Output:

```
Hello, world!
```

## Other commands

```bash
cargo build     # compile only, binary lands in target/debug/hello_cargo
cargo check     # type-check without producing a binary (much faster)
cargo build --release   # optimized build, into target/release/
```

## Layout

```
Cargo.toml    package name, version, edition, dependencies
Cargo.lock    exact dependency versions (committed, since this is a binary)
src/main.rs   entry point
target/       build output (gitignored)
```

Cargo expects source in `src/`, which is why `main.rs` lives there rather than
next to `Cargo.toml`.
