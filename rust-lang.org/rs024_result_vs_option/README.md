| Feature                    | **Option**                                                         | **Result**                                                                             |
| :------------------------- | :----------------------------------------------------------------- | :------------------------------------------------------------------------------------- |
| **Purpose**                | Represents the **absence** of a value (nullability).               | Represents the **outcome** of a fallible operation (error handling).                   |
| **Variants**               | `Some(T)`: Value exists.<br>`None`: No value.                      | `Ok(T)`: Operation succeeded.<br>`Err(E)`: Operation failed with error `E`.            |
| **Semantic Meaning**       | "Maybe there is a value, maybe there isn't."                       | "The operation worked, or it failed for a specific reason."                            |
| **Error Information**      | **None**: `None` carries no data about _why_ the value is missing. | **Specific**: `Err` contains data describing the failure (e.g., `io::Error`).          |
| **Common Use Cases**       | Searching a collection, optional struct fields, head of a list.    | File I/O, network requests, parsing, database connections.                             |
| **Failure Modes**          | Single failure mode (value missing).                               | Multiple potential failure modes (network error, permission denied, not found, etc.).  |
| **Key Conversion Methods** | `ok_or(E)`: Converts to `Result`.                                  | `ok()`: Converts to `Option` (discards error).<br>`err()`: Extracts error as `Option`. |
| **Unwrapping**             | `unwrap()`: Panics if `None`.                                      | `unwrap()`: Panics if `Err`.                                                           |
