use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_directory = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("Cargo must provide CARGO_MANIFEST_DIR"),
    );
    let ghostty_source = manifest_directory.join("vendor/ghostty");
    assert!(
        ghostty_source.join("build.zig").is_file(),
        "Ghostty source is missing; initialize the vendor/ghostty submodule"
    );

    let output_directory = PathBuf::from(
        env::var_os("OUT_DIR").expect("Cargo must provide OUT_DIR for build scripts"),
    );
    let install_prefix = output_directory.join("ghostty-vt");
    build_libghostty_vt(&ghostty_source, &install_prefix);

    println!(
        "cargo:rustc-link-search=native={}",
        install_prefix.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=ghostty-vt");
    println!("cargo:rerun-if-env-changed=ZIG");
    println!("cargo:rerun-if-changed={}", ghostty_source.display());
}

fn build_libghostty_vt(source: &Path, install_prefix: &Path) {
    let zig = env::var_os("ZIG").unwrap_or_else(|| OsString::from("zig"));
    let optimization = match env::var("OPT_LEVEL")
        .expect("Cargo must provide OPT_LEVEL")
        .as_str()
    {
        "0" => "Debug",
        _ => "ReleaseFast",
    };

    let status = Command::new(&zig)
        .current_dir(source)
        .arg("build")
        // Zig's cache is content-addressed and locks each entry, so Zig's global
        // cache can serve every build of this crate: a new Cargo output directory
        // reuses the library compiled before instead of rebuilding it. Zig's
        // default, a cache beside build.zig, would write into the source, which
        // Cargo may keep read-only.
        .arg("--cache-dir")
        .arg(zig_global_cache(&zig, source))
        .arg("-Demit-lib-vt")
        .arg("-Demit-xcframework=false")
        .arg(format!("-Doptimize={optimization}"))
        .args(zig_target().map(|target| format!("-Dtarget={target}")))
        .arg("--prefix")
        .arg(install_prefix)
        .status()
        .expect("failed to launch Zig while building libghostty-vt");

    assert!(status.success(), "libghostty-vt build failed");
}

/// The target Zig builds for: `GHOSTTY_VT_ZIG_TARGET` when set, which names
/// it exactly (such as `x86_64-linux-gnu.2.28` for an older glibc); else
/// Cargo's target when it is not the machine building; else none, and Zig
/// builds for this machine.
fn zig_target() -> Option<String> {
    println!("cargo:rerun-if-env-changed=GHOSTTY_VT_ZIG_TARGET");
    if let Some(target) = env::var_os("GHOSTTY_VT_ZIG_TARGET") {
        return Some(
            target
                .into_string()
                .expect("GHOSTTY_VT_ZIG_TARGET names a Zig target in ASCII"),
        );
    }
    let cargo = |name: &str| env::var(name).unwrap_or_else(|_| panic!("Cargo must provide {name}"));
    if cargo("HOST") == cargo("TARGET") {
        return None;
    }
    let architecture = cargo("CARGO_CFG_TARGET_ARCH");
    let os = cargo("CARGO_CFG_TARGET_OS");
    let abi = cargo("CARGO_CFG_TARGET_ENV");
    Some(if abi.is_empty() {
        format!("{architecture}-{os}")
    } else {
        format!("{architecture}-{os}-{abi}")
    })
}

/// Zig's global cache directory, as `zig env` reports it when run in `directory`.
fn zig_global_cache(zig: &OsStr, directory: &Path) -> PathBuf {
    let output = Command::new(zig)
        .current_dir(directory)
        .arg("env")
        .output()
        .expect("failed to launch Zig to find its global cache");
    assert!(
        output.status.success(),
        "`zig env` failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report = String::from_utf8(output.stdout).expect("`zig env` prints ASCII");
    let literal = report
        .lines()
        .find_map(|line| line.trim_start().strip_prefix(".global_cache_dir = \""))
        .expect("`zig env` reports .global_cache_dir");
    // Zig names the directory relative to the working directory when it lies
    // inside it.
    directory.join(decode_zon_string(literal))
}

/// Decodes a ZON string literal that starts after its opening quote. `zig env`
/// writes printable ASCII as is and escapes every other byte.
fn decode_zon_string(literal: &str) -> String {
    let mut bytes = Vec::new();
    let mut rest = literal.bytes();
    loop {
        let byte = match rest.next().expect("`zig env` closes every string") {
            b'"' => break,
            b'\\' => match rest.next() {
                Some(b'n') => b'\n',
                Some(b'r') => b'\r',
                Some(b't') => b'\t',
                Some(b'x') => {
                    let digits = [rest.next(), rest.next()]
                        .map(|digit| digit.expect("`zig env` writes two digits after \\x"));
                    let digits = std::str::from_utf8(&digits).expect("hex digits are ASCII");
                    u8::from_str_radix(digits, 16).expect("`zig env` writes hex digits after \\x")
                }
                Some(escaped @ (b'\\' | b'"')) => escaped,
                escape => panic!("unexpected escape in `zig env` output: {escape:?}"),
            },
            byte => byte,
        };
        bytes.push(byte);
    }
    String::from_utf8(bytes).expect("Zig's global cache path must be UTF-8")
}
