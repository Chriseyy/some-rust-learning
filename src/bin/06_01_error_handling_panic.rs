
// Error handeling in Rust

// unrecoverbale errore with panic !
// this means that the program will stop execution and print an error message to the console. This is useful for situations where the program cannot continue, such as when a file is not found or when a network connection is lost.

fn run_panic() {
    panic!("This is a panic message!"); // this will cause the program to stop execution and print the panic message to the console
}

fn after_panic() {
    println!("This will not be printed because the program has panicked and stopped execution.");
}

fn  main() {
    println!("Hello, world!");
    run_panic();
    after_panic(); // this will not be executed because the program has panicked and stopped execution.
}
