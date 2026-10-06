# hello_world

The classic first Rust program, built directly with `rustc` — no Cargo.

## Build

```bash
rustc main.rs
```

This compiles `main.rs` into an executable named `main` in the same folder.

## Run

```bash
./main
```

Output:

```
Hello world
```

## Notes

- `rustc` names the binary after the source file. Use `-o <name>` to pick your own:
  ```bash
  rustc main.rs -o hello
  ```
- Builds are unoptimized by default. Add `-O` for an optimized binary.
- The compiled `main` binary is a build artifact — worth adding to `.gitignore` rather than committing.
