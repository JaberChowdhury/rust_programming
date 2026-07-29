// CONCEPT: Understand why `Pin` exists through a self-referential struct example.
// WHY: Async blocks often compile down to state machines that contain references to their own internal fields. `Pin` guarantees they aren't moved in memory.

use std::pin::Pin;
use std::marker::PhantomPinned;
use tokio_stream::StreamExt;

// A self-referential struct simulation
struct SelfReferential {
    data: String,
    // pointer to data
    pointer: *const String,
    _marker: PhantomPinned,
}

impl SelfReferential {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::new(SelfReferential {
            data,
            pointer: std::ptr::null(),
            _marker: PhantomPinned,
        });
        
        let ptr: *const String = &boxed.data;
        
        // Unsafe: we are modifying the struct to point to its own heap allocation.
        // Once pinned, this is safe as long as it isn't moved.
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::new_unchecked(&mut *boxed);
            let unpinned = Pin::get_unchecked_mut(mut_ref);
            unpinned.pointer = ptr;
        }
        
        boxed.into()
    }
    
    fn print_data(self: Pin<&Self>) {
        unsafe {
            println!("Data via pointer: {}", &*self.pointer);
        }
    }
}

#[tokio::main]
async fn main() {
    let pinned_struct = SelfReferential::new("Hello, Pin!".to_string());
    pinned_struct.as_ref().print_data();

    // tokio::pin! macro for stack pinning
    let future = async {
        println!("Running pinned future");
    };
    tokio::pin!(future); // Pins `future` to the stack
    future.await;

    // Stream that requires pinning
    let stream = tokio_stream::iter(vec![1, 2, 3]);
    tokio::pin!(stream);
    while let Some(val) = stream.next().await {
        println!("Stream value: {}", val);
    }
}

// EXERCISE:
// 1. Try to un-pin the `stream` above by removing `tokio::pin!(stream)` and observe the compiler error about `Unpin`.
// 2. Try to move `pinned_struct` to a new variable and see if it can still be used.
