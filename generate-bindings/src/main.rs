//! Regenerates `src/bindings.rs` from the pinned libghostty-vt headers.
//!
//! Run `cargo run -p generate-bindings` after moving `vendor/ghostty` and
//! commit the result. With `--check`, it changes nothing and fails when the
//! committed bindings differ from the headers' current output.

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};
use std::thread;

fn main() -> ExitCode {
    let check = match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => {
            eprintln!("usage: generate-bindings [--check]");
            return ExitCode::FAILURE;
        }
    };

    let crate_directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the generator lives inside the ghostty-vt-sys repository");
    let bindings = format(generate(&crate_directory.join("vendor/ghostty/include")));
    let path = crate_directory.join("src/bindings.rs");

    if check {
        let committed = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        if committed != bindings {
            eprintln!(
                "{} does not match the pinned headers; run `cargo run -p generate-bindings`",
                path.display()
            );
            return ExitCode::FAILURE;
        }
    } else {
        fs::write(&path, bindings)
            .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
    }
    ExitCode::SUCCESS
}

fn generate(include: &Path) -> String {
    bindgen::Builder::default()
        .header(include.join("ghostty/vt.h").to_string_lossy())
        // C++11 selects the headers' explicit `enum : int` declarations, matching
        // Ghostty's Zig ABI even on Clang versions whose C enums are unsigned.
        .clang_args(["-x", "c++", "-std=c++11"])
        .clang_arg(format!("-I{}", include.display()))
        .allowlist_function("ghostty_.*")
        .allowlist_type("Ghostty.*")
        .allowlist_var("GHOSTTY_.*")
        // The headers' enum-sizing sentinel is `INT_MAX`, and whether bindgen
        // emits it varies between toolchains (Apple clang 17 does, LLVM 21 on
        // Linux doesn't). The bindings must be the same everywhere; each enum's
        // own `*_MAX_VALUE` member still carries the value.
        .blocklist_var("GHOSTTY_ENUM_MAX_VALUE")
        .prepend_enum_name(false)
        .generate_comments(false)
        // `format` below runs rustfmt and fails loudly; bindgen's own rustfmt step
        // would silently write unformatted bindings when rustfmt is missing.
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("failed to generate bindings from the pinned libghostty-vt headers")
        .to_string()
}

fn format(source: String) -> String {
    let mut rustfmt = Command::new(env::var_os("RUSTFMT").unwrap_or_else(|| "rustfmt".into()))
        .args(["--edition", "2024"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to launch rustfmt");
    let mut input = rustfmt.stdin.take().expect("rustfmt's stdin is piped");
    // Write from another thread so rustfmt never blocks on a full stdout pipe.
    let writer = thread::spawn(move || input.write_all(source.as_bytes()));
    let output = rustfmt
        .wait_with_output()
        .expect("failed to wait for rustfmt");
    writer
        .join()
        .expect("the rustfmt writer thread panicked")
        .expect("failed to pass the bindings to rustfmt");
    assert!(output.status.success(), "rustfmt failed on the bindings");
    String::from_utf8(output.stdout).expect("rustfmt prints UTF-8")
}
