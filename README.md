# ghostty-vt-sys

Raw Rust FFI declarations and native build integration for
[`libghostty-vt`](https://github.com/ghostty-org/ghostty).

This repository owns three things:

- the exact Ghostty source revision, pinned as `vendor/ghostty`;
- compiling Ghostty's static `ghostty-vt` library with Zig;
- a reviewed subset of the matching C declarations.

It intentionally contains no safe wrapper or application-specific types. Those
belong to consuming crates.

## Requirements

- Rust with edition 2024 support;
- Zig compatible with the pinned Ghostty revision (tested with Zig 0.15.2);
- a native build (`HOST == TARGET`). Cross-compilation is rejected explicitly.

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
2. Review every declaration in `src/lib.rs` against the headers under
   `vendor/ghostty/include/ghostty/vt`.
3. Run formatting, tests, and Clippy from a clean checkout.
4. Pin consumers to the resulting `ghostty-vt-sys` commit.

Ghostty is licensed under the MIT License; see `vendor/ghostty/LICENSE`.

