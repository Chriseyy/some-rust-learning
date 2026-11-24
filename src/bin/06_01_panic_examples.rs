// Error handling in Rust: Unrecoverable errors with `panic!`
//
// `panic!` stops execution immediately and prints an error message.
// Use it for unrecoverable bugs or broken assumptions (e.g., invalid state,
// out-of-bounds access)—NOT for expected failures like missing files or
// dropped network connections (those should use `Result<T, E>`).
//
// Debugging tip: Run with a backtrace to see the exact call stack on panic:
//   RUST_BACKTRACE=1 cargo run

fn run_panic() {
    panic!("This is a panic message!"); // Stops execution immediately
}

fn after_panic() {
    println!("This will not be printed because the program has panicked.");
}

fn main() {
    println!("Hello, world!");
    run_panic();
    after_panic(); // Never reached
}