// =============================================================================
// Getting Started: Installation, Hello World & Cargo (Chapter 1)
// =============================================================================
//
// 1. INSTALLATION & TOOLING (`rustup`)
//    - `rustup`: Manages Rust versions and associated tools.
//    - `rustc --version`   -> Check installed Rust compiler version.
//    - `rustup update`     -> Update to the latest stable Rust version.
//    - `rustup doc`        -> Open local offline API documentation in browser.
//    - `rustup self uninstall` -> Uninstall Rust and rustup.
//    - `rustfmt`           -> Automatic code formatter (included with Rust).
//
// 2. COMPILATION (`rustc` — Ahead-of-Time Compiled)
//    - Rust compiles directly to a standalone binary executable (no runtime needed).
//    - Manual compile: `rustc main.rs` -> creates `./main` (or `.\main.exe` on Windows).
//    - Naming convention: `snake_case.rs` (e.g., `hello_world.rs`).
//
// 3. CARGO (Build System & Package Manager)
//    - `cargo new <name>`    -> Creates new project (`Cargo.toml`, `src/main.rs`, Git repo).
//    - `cargo init`          -> Adds Cargo to an existing directory.
//    - `Cargo.toml`          -> Project config (name, version, edition, `[dependencies]`).
//    - `Cargo.lock`          -> Auto-generated; tracks exact dependency versions (do not edit).
//
//    Everyday Commands:
//    - `cargo check`         -> Fast compile check WITHOUT creating an executable.
//    - `cargo build`         -> Compiles debug binary into `target/debug/`.
//    - `cargo run`           -> Compiles (if changed) and runs the binary in one step.
//    - `cargo build --release` -> Optimized build in `target/release/` (slower compile, faster runtime).
// =============================================================================

// `fn main()` is always the entry point of every executable Rust program.
fn main() {
    // `println!` calls a Rust macro (note the `!`), not a normal function.
    // Most statements in Rust end with a semicolon (`;`).
    println!("Hello, world!");
}