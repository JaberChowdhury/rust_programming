// CONCEPT: Actor model pattern using channels.

mod mpsc_demo;
mod broadcast_demo;
mod watch_demo;
mod oneshot_demo;

#[tokio::main]
async fn main() {
    println!("--- MPSC Demo ---");
    mpsc_demo::run().await;
    
    println!("--- Oneshot Demo ---");
    oneshot_demo::run().await;

    println!("--- Broadcast Demo ---");
    broadcast_demo::run().await;

    println!("--- Watch Demo ---");
    watch_demo::run().await;
}
// EXERCISE:
// 1. Create a channel pipeline.
