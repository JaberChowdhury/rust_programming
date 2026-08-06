// CONCEPT: Process a sequence of async values.
// WHY: Streams are the async equivalent of iterators. They are crucial for processing paginated APIs, websockets, or event logs.

use async_stream::stream;
use tokio::time::{sleep, Duration};
use tokio_stream::StreamExt;

async fn fetch_page(page: usize) -> Option<Vec<usize>> {
    sleep(Duration::from_millis(100)).await;
    if page <= 3 {
        Some(vec![page * 10, page * 10 + 1, page * 10 + 2])
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    // 1. Stream from a Vec
    println!("--- Stream from Vec ---");
    let mut vec_stream = tokio_stream::iter(vec![1, 2, 3]);
    while let Some(v) = vec_stream.next().await {
        println!("Vec stream: {}", v);
    }

    // 2. Custom stream using async-stream macro (simulating paginated API)
    println!("\n--- Custom Stream (Paginated API) ---");
    let api_stream = stream! {
        let mut page = 1;
        while let Some(data) = fetch_page(page).await {
            for item in data {
                yield item; // Yield items one by one
            }
            page += 1;
        }
    };

    tokio::pin!(api_stream); // Needed for stream! macro

    while let Some(v) = api_stream.next().await {
        println!("API item: {}", v);
    }

    // 3. Stream combinators
    println!("\n--- Stream Combinators ---");
    let mut mapped_stream = tokio_stream::iter(1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * 10)
        .take(3);

    while let Some(v) = mapped_stream.next().await {
        println!("Mapped: {}", v);
    }
}

// EXERCISE:
// 1. Add `.timeout(Duration::from_millis(50))` to `api_stream` (you'll need to import the trait or rely on StreamExt) and see it fail.
// 2. Change the `take(3)` to `take(5)` and see the output.
