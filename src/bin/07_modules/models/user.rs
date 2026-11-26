// PUBLIC ENUM vs. PUBLIC STRUCT 
// Making an enum `pub` automatically makes ALL of its variants public.
// (Unlike `pub struct`, where individual fields remain private by default.)
#[derive(Debug)]  // this makes the enum printable with `{:?}` in `println!()`
pub enum AccountStatus {
    Active, // Active = 0, Suspended = 1  and 1 byte in memory, but we don't care about the actual values here
    Suspended,      // own type AccountStatus, so we can use it as a field in our `User` struct
}   // all variants are public automatically, so `main.rs` can use them without `pub` on each variant

// #[derive(Debug)] creates something like this behind the scenes:
// match self {
//     AccountStatus::Active => "Active",
//     AccountStatus::Suspended => "Suspended",
// }

// A `pub struct` keeps its fields private by default.
#[derive(Debug)]
pub struct User {  
    pub name: String, // Public: readable and writable from outside (needs `pub` for `main.rs` to access it)
    pub role: String,          // Public
    pub status: AccountStatus, // Public field using our public enum
    password: String,          // Private: only accessible inside `user.rs`
}

impl User {
    // Public constructor (required because `password` is private)
    pub fn new(name: &str, password: &str) -> Self {
        Self {
            name: String::from(name),
            role: super::default_role(), // Relative path with `super::` -> parent module (`models.rs`)
            status: AccountStatus::Active,
            password: String::from(password),
        }
    }

    pub fn check_password(&self, attempt: &str) -> bool {
        self.password == attempt
    }
}