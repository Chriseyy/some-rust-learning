// Packages, Crates, and Modules (Chapter 7)
// - Packages: A Cargo feature that lets you build, test, and share crates.
// - Crates: A tree of modules that produces a library or executable.
// - Modules & `use`: Control the organization, scope, and privacy of paths.
// - Paths: A way of naming an item, such as a struct, function, or module.
//
// - Library crate (`src/lib.rs`): Defines functionality shared with other projects.
// - Binary crate (`src/main.rs`): Defines an executable program that runs on its own.

// 1. Register files as modules in the crate tree (compiler loads `math.rs` and `models.rs`)
mod math;
mod models;

//  Inline module: defined directly inside this file using `{ ... }` instead of a separate file.
//    Useful for small groupings or unit tests. Same privacy rules apply (`pub` vs. private).
mod greetings {
    pub fn say_hi(name: &str) {
        println!("Hi, {name}! {}", secret_suffix());
    }

    fn secret_suffix() -> &'static str {
        "(from inline module)"
    }
}

// 2. Bring items into scope with `use` (absolute path starting from `crate::`)
// use crate::models::user::User;  // `User` struct is now accessible without the full path
// instead of crate::models::user::User::new(...) 
use crate::models::user::{AccountStatus, User};

// 3. Aliasing with `as` (useful to avoid name conflicts)
use crate::math::add_with_bonus as add_bonus;   // `add_with_bonus` is now accessible as `add_bonus`


fn main() {
    // --- ABSOLUTE vs. RELATIVE PATHS (Chapter 7.3) ---
    // Absolute path: starts from the crate root (`crate::`), like `/path` in a filesystem.
    let sum_abs = crate::math::add_with_bonus(5, 7);

    // Relative path: starts from the current module (`math` is a sibling of `main`).
    let sum_rel = math::add_with_bonus(5, 7);

    // Shortcut via `use ... as`
    let sum_alias = add_bonus(5, 7);
    println!("Sums (abs / rel / alias): {sum_abs} / {sum_rel} / {sum_alias}");

    // --- PUBLIC vs. PRIVATE STRUCT FIELDS ---
    let mut user = User::new("Alice", "rust123");
    user.name = String::from("Alicia"); // Allowed: `name` is `pub`
    // user.password = String::from("hack"); // ERROR: `password` is private

    // --- PUBLIC ENUM (all variants are public automatically) ---
    user.status = AccountStatus::Suspended;

    println!("User: {} | Role: {} | Status: {:?}", user.name, user.role, user.status);
    println!("Password valid? {}", user.check_password("rust123"));

    // --- INLINE MODULE ---
    greetings::say_hi(&user.name);
}