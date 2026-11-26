

// `pub` makes this function accessible from other modules (like `main.rs`).
pub fn add_with_bonus(a: i32, b: i32) -> i32 {
    a + b + secret_bonus()
}

// Private by default: only code inside `math.rs` can call this.
fn secret_bonus() -> i32 {
    10
}