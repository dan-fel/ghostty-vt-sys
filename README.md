# ghostty-vt-sys

[![CI](https://github.com/dan-fel/ghostty-vt-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/dan-fel/ghostty-vt-sys/actions/workflows/ci.yml)

Raw Rust bindings to [libghostty-vt](https://github.com/ghostty-org/ghostty),
Ghostty's virtual terminal library. This crate pins the Ghostty source, builds its
static library with Zig, and generates matching Rust declarations with bindgen.

Use it to embed Ghostty's terminal parser and state in a Rust application. Safe
wrappers, PTY/process management, and rendering belong in the consuming application.
This is an independent integration, not an official Ghostty project.

## Status and supported builds

The crate is distributed as a Git dependency; it is not published to crates.io.
The upstream C API is still evolving, and updating the pinned Ghostty revision can
break Rust callers. Pin an exact crate commit and review updates before adopting
them. The package version alone does not identify the upstream ABI.

CI validates native builds on **macOS and Linux**. Other platforms are not
currently tested. Cross-compilation, including WebAssembly, is not supported:
the build explicitly requires `HOST == TARGET`.

## Requirements

- Stable Rust with edition 2024 support. CI tracks the stable toolchain; older
  Rust versions are not currently validated.
- Zig compatible with the pinned Ghostty revision (currently **0.16.0**).
- Clang/libclang for bindgen: Xcode Command Line Tools on macOS, or
  `libclang-dev` on Debian/Ubuntu.

The build uses `zig` from PATH; `ZIG` selects another executable.
`LIBCLANG_PATH` can select a libclang installation. The pinned Ghostty source
checks its required Zig version in `vendor/ghostty/build.zig.zon`.

The first build downloads Ghostty's Zig dependencies and compiles the native
library. Cargo and Zig cache subsequent builds.

## Getting started

Clone with submodules, then run the example:

```sh
git clone --recurse-submodules https://github.com/dan-fel/ghostty-vt-sys.git
cd ghostty-vt-sys
cargo run --example basic
```

The [example](examples/basic.rs) creates a terminal, writes styled text, reads the
cursor position, and frees the terminal. It prints:

```text
Cursor column after writing styled text: 5
```

For an existing checkout with a missing `vendor/ghostty`, run:

```sh
git submodule update --init --recursive
```

To depend on this crate, add the following to your application's `Cargo.toml`,
replacing `<commit>` with a full SHA from the
[crate history](https://github.com/dan-fel/ghostty-vt-sys/commits/main):

```toml
[dependencies.ghostty-vt-sys]
git = "https://github.com/dan-fel/ghostty-vt-sys.git"
rev = "<commit>"
```

Cargo fetches the pinned Git submodule along with the crate. Rust imports use the
name `ghostty_vt_sys`.

## API and safety contracts

The [pinned C headers](https://github.com/ghostty-org/ghostty/blob/09a2724c23fd13f7cd24c093c568a4b6792a66a2/include/ghostty/vt.h)
document the API and are the source of truth for the generated declarations.
Generate local Rust API reference pages with `cargo doc --no-deps --open`.

These are raw FFI bindings. Callers must pair creation and destruction correctly,
keep borrowed buffers and callback userdata alive for their documented lifetimes,
and serialize access to terminal instances. Each query or option selector defines
the required pointer type; sized structs must have their `size` field initialized.
Callbacks must use the declared C calling convention and must not unwind across
the FFI boundary.

Bindings are generated into Cargo's build output, with compile-time C layout
assertions. The headers are parsed as C++11 so their explicitly signed enums match
Ghostty's Zig ABI across the supported Clang toolchains. This does not introduce a
C++ runtime dependency. An integration test also compares embedding layouts and
selected enum values against the compiled library's ABI manifest.

C function macros and static inline mode helpers are not emitted. `GhosttyMode`
uses bits 0–14 for the mode number and bit 15 for ANSI modes; leave bit 15 clear
for DEC private modes.

## Development

Run the same checks as CI:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo run --example basic
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo clippy --workspace --all-targets -- -D warnings
```

For contributions, keep the crate focused on raw bindings and native build
integration. Include a regression test for behavior changes. Bug reports should
include the crate revision, OS/architecture, `rustc --version`, `zig version`, and
a minimal reproduction.

### Updating Ghostty

1. Move `vendor/ghostty` to the intended upstream commit.
2. Review header changes and adapt the integration tests and example. Declarations
   regenerate from `vendor/ghostty/include/ghostty/vt.h`.
3. Check `vendor/ghostty/build.zig.zon` for toolchain changes and update the CI Zig
   version, requirements, and pinned upstream links above when needed.
4. Run the development checks and confirm the macOS and Linux CI jobs pass.
5. Pin consumers to the resulting `ghostty-vt-sys` commit and validate their
   integration against the new API.

## License

This crate is licensed under the [MIT License](LICENSE).
[Ghostty](https://github.com/ghostty-org/ghostty/blob/09a2724c23fd13f7cd24c093c568a4b6792a66a2/LICENSE)
is also MIT-licensed and retains its own copyright notice in
`vendor/ghostty/LICENSE`.
