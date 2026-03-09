Minimal example of redirecting `printf`/`fprintf` from C bindings to Rust's [`log`](https://crates.io/crates/log) crate.

Works on stable Rust.

## How it works

- `redirect.h` redefines `printf`/`fprintf` as custom C functions via `#define`
- `build.rs` force-includes `redirect.h` into C sources using `-include` flag (no changes to C source files needed)
- The custom functions call back into Rust via FFI, where output is passed to `log`

## Usage

```
RUST_LOG=debug cargo run
```

## Limitations

For simplicity, this minimal example does not buffer partial prints. If the C code prints a single line across multiple `printf` calls (e.g., `printf("Loading... "); printf("Done!\n");`), each call will be emitted as separate log events in Rust. In practice, you should decide whether to manage global state in Rust depending on the specific C library you're binding.

## Example Project

This technique is used in [Haqumei](https://github.com/stellanomia/haqumei), a Japanese G2P engine written in Rust, to suppress and redirect logs from its underlying C dependencies (Open JTalk).

## License

MIT
