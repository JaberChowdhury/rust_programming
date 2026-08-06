// CONCEPT: Master error propagation in async code.
// WHY: Error handling in async functions behaves much like sync Rust, but task spawning introduces extra layers of `Result`.

use anyhow::Context;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("simulated network failure")]
    NetworkError,
    #[error("data not found")]
    NotFound,
}

async fn fetch_data(fail: bool) -> Result<String, MyError> {
    if fail {
        Err(MyError::NetworkError)
    } else {
        Ok("success data".to_string())
    }
}

async fn process_data(fail: bool) -> anyhow::Result<String> {
    // 1. Using `?` operator and wrapping with context
    let data = fetch_data(fail)
        .await
        .context("Failed to fetch data during processing")?;
    Ok(format!("Processed: {}", data))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Successful case
    match process_data(false).await {
        Ok(d) => println!("Success: {}", d),
        Err(e) => println!("Error: {:?}", e), // Print full anyhow context
    }

    // Failure case
    match process_data(true).await {
        Ok(d) => println!("Success: {}", d),
        Err(e) => println!("Error: {:?}", e),
    }

    // Error in spawned task (Double Result)
    let handle = tokio::spawn(async { fetch_data(true).await });

    // The `.await` returns a Result<Result<String, MyError>, JoinError>
    let join_result = handle.await;
    match join_result {
        Ok(Ok(val)) => println!("Task succeeded: {}", val),
        Ok(Err(e)) => println!("Task failed with logic error: {}", e),
        Err(e) => println!("Task panicked or cancelled: {}", e),
    }

    Ok(())
}

// EXERCISE:
// 1. Make the spawned task panic (`panic!("boom")`) instead of returning an error, and observe the `JoinError`.
// 2. Use `unwrap_or_else` on `fetch_data` to provide a fallback string on failure.
