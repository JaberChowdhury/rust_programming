// CONCEPT: Async streams and tokio-stream.

use async_stream::stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);

    tokio::spawn(async move {
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
    });

    let mut stream1 = ReceiverStream::new(rx);
    while let Some(v) = stream1.next().await {
        println!("ReceiverStream: {}", v);
    }

    let stream2 = stream! {
        for i in 0..3 {
            yield i;
        }
    };
    tokio::pin!(stream2);

    while let Some(v) = stream2.next().await {
        println!("Custom Stream: {}", v);
    }
}
// EXERCISE:
// 1. Use .timeout() on a stream.
