use std::env;
use std::ffi::OsString;
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

    let host = env::var("HOST").expect("Cargo must provide HOST");
    let target = env::var("TARGET").expect("Cargo must provide TARGET");
    assert_eq!(
        host, target,
        "ghostty-vt-sys does not currently support cross-compilation"
    );

    let output_directory = PathBuf::from(
        env::var_os("OUT_DIR").expect("Cargo must provide OUT_DIR for build scripts"),
    );
    bindgen::Builder::default()
        .header(
            ghostty_source
                .join("include/ghostty/vt.h")
                .to_string_lossy(),
        )
        // C++11 selects the headers' explicit `enum : int` declarations, matching
        // Ghostty's Zig ABI even on Clang versions whose C enums are unsigned.
        .clang_args(["-x", "c++", "-std=c++11"])
        .clang_arg(format!("-I{}", ghostty_source.join("include").display()))
        .allowlist_function("ghostty_.*")
        .allowlist_type("Ghostty.*")
        .allowlist_var("GHOSTTY_.*")
        .prepend_enum_name(false)
        .generate_comments(false)
        // Generated build output is compiled directly; it needs no pretty printer.
        .formatter(bindgen::Formatter::None)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate bindings from the pinned libghostty-vt headers")
        .write_to_file(output_directory.join("bindings.rs"))
        .expect("failed to write libghostty-vt bindings");

    let install_prefix = output_directory.join("ghostty-vt");
    build_libghostty_vt(&ghostty_source, &output_directory, &install_prefix);

    println!(
        "cargo:rustc-link-search=native={}",
        install_prefix.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=ghostty-vt");
    println!("cargo:rerun-if-env-changed=ZIG");
    println!("cargo:rerun-if-changed={}", ghostty_source.display());
}

fn build_libghostty_vt(source: &Path, output_directory: &Path, install_prefix: &Path) {
    let zig = env::var_os("ZIG").unwrap_or_else(|| OsString::from("zig"));
    let optimization = match env::var("OPT_LEVEL")
        .expect("Cargo must provide OPT_LEVEL")
        .as_str()
    {
        "0" => "Debug",
        _ => "ReleaseFast",
    };

    let status = Command::new(zig)
        .current_dir(source)
        .env(
            "ZIG_LOCAL_CACHE_DIR",
            output_directory.join("zig-local-cache"),
        )
        .arg("build")
        .arg("-Demit-lib-vt")
        .arg("-Demit-xcframework=false")
        .arg(format!("-Doptimize={optimization}"))
        .arg("--prefix")
        .arg(install_prefix)
        .status()
        .expect("failed to launch Zig while building libghostty-vt");

    assert!(status.success(), "libghostty-vt build failed");
}
