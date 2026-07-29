// CONCEPT: Correct Tokio setup and feature flags.

use tokio::runtime::Handle;

// WHY: #[tokio::main] macro creates a runtime and blocks on the async main function.
#[tokio::main]
async fn main() {
    println!("Tokio setup works!");
    let handle = Handle::current();
    println!("Got runtime handle: {:?}", handle);
}
// EXERCISE:
// 1. Try removing #[tokio::main] and see what happens.
// 2. Try #[tokio::main(flavor = "current_thread")]
