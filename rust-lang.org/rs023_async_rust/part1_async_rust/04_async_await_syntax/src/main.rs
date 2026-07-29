// CONCEPT: Show how `async fn` desugars and how `.await` works.
// WHY: To demystify async syntax and understand that async functions are lazy.

async fn fetch_data() -> String {
    "Data".to_string()
}
// Desugared equivalent roughly looks like:
// fn fetch_data() -> impl std::future::Future<Output = String> {
//     async { "Data".to_string() }
// }

async fn process_data(data: String) -> Result<String, ()> {
    Ok(format!("Processed {}", data))
}

#[tokio::main]
async fn main() {
    // 1. Lazy evaluation: calling async fn does nothing by itself
    let _future = fetch_data();
    println!("Called fetch_data(), but it hasn't run yet.");

    // 2. Chaining multiple `.await` calls
    let result = process_data(fetch_data().await).await;
    match result {
        Ok(s) => println!("Result: {}", s),
        Err(_) => println!("Error"),
    }

    // 3. async blocks
    let async_block = async {
        println!("Inside async block");
        42
    };
    let val = async_block.await;
    println!("Async block returned: {}", val);
}

// EXERCISE:
// 1. Create a function returning `()` and `.await` it.
// 2. Try removing an `.await` on a returned Future and try to print it. Observe the compiler error.
