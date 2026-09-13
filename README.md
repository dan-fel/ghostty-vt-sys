# ghostty-vt-sys

Raw Rust FFI declarations and native build integration for
[`libghostty-vt`](https://github.com/ghostty-org/ghostty).

This repository owns three things:

- the exact Ghostty source revision, pinned as `vendor/ghostty`;
- compiling Ghostty's static `ghostty-vt` library with Zig;
- generating raw Rust declarations from the matching C headers with bindgen.

It intentionally contains no safe wrapper or application-specific types. Those
belong to consuming crates.

## Requirements

- Rust with edition 2024 support;
- Zig compatible with the pinned Ghostty revision (currently Zig 0.16.0);
- Clang/libclang for bindgen (Xcode Command Line Tools on macOS, or
  `libclang-dev` on Debian/Ubuntu; `LIBCLANG_PATH` can select an installation);
- a native build (`HOST == TARGET`). Cross-compilation is rejected explicitly.

The build uses `zig` from PATH. Set `ZIG` to select another executable.
The pinned Ghostty source enforces its Zig version requirement.

Bindings are generated into Cargo's build output, including C layout assertions.
The headers are parsed as C++11 so their explicitly signed enums match Ghostty's
Zig ABI on every supported Clang version. This does not introduce a C++ runtime
dependency. C-only function macros and static inline mode helpers are not emitted;
`GhosttyMode` uses bits 0–14 for the mode number and bit 15 for ANSI modes (clear
for DEC private modes).

Clone this repository with its submodule before developing it directly:

```sh
git clone --recurse-submodules git@github.com:dan-fel/ghostty-vt-sys.git
cargo test
```

Consumers should pin an exact repository revision:

```toml
ghostty-vt-sys = {
    git = "https://github.com/dan-fel/ghostty-vt-sys.git",
    rev = "<commit>"
}
```

## Updating Ghostty

1. Move `vendor/ghostty` to the intended upstream commit.
2. Review upstream header changes and adapt the integration tests. The raw
   declarations regenerate from `vendor/ghostty/include/ghostty/vt.h`; the ABI test
   compares the embedding structs against the native library's type manifest.
3. Run `cargo fmt --all -- --check`, `cargo test --workspace`, and
   `cargo clippy --workspace --all-targets -- -D warnings` from a clean checkout.
   Use the Zig version required by `vendor/ghostty/build.zig.zon`.
4. Pin consumers to the resulting `ghostty-vt-sys` commit.

Ghostty is licensed under the MIT License; see `vendor/ghostty/LICENSE`.

