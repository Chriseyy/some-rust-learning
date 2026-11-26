// =============================================================================
// WHY DOES `models.rs` + THE `models/` FOLDER EXIST?
// -----------------------------------------------------------------------------
// Rust does not automatically scan subfolders. `models.rs` acts as the entry
// point ("table of contents") for the `models/` directory.
//
// ADVANTAGE (Organization in larger projects):
//   - Groups related files cleanly (e.g., `models/user.rs`, `models/product.rs`).
//   - Keeps `main.rs` clean: `main.rs` only needs one line (`mod models;`)
//     instead of listing every single file.
//
// WITHOUT A SUBFOLDER (for very small projects):
//   - You could skip `models.rs` completely, place `user.rs` directly next to
//     `main.rs`, and just write `mod user;` inside `main.rs`.
// =============================================================================

// Loads `models/user.rs` and makes it public so `main.rs` can access it.
// Without `pub`, only `models.rs` itself could use `user`.
pub mod user;

// Private helper function: hidden from `main.rs`, but child modules
// (like `models/user.rs`) can still access it via `super::default_role()`.
fn default_role() -> String {
    String::from("Member")
}