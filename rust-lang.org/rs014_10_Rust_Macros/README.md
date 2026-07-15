### 1. `include_str!` (0:23)

Embeds a text file into the binary at compile time. Use this to avoid runtime I/O errors.

- **Example:** `const QUERY: &str = include_str!("query.sql");`

### 2. `include_bytes!` (1:16)

Similar to `include_str!`, but for binary data like images or certificates.

- **Example:** `const ICON: &[u8] = include_bytes!("icon.png");`

### 3. `concat!` (1:36)

Concatenates string literals at compile time with zero runtime cost.

- **Example:** `const PATH: &str = concat!(env!("HOME"), "/config");`

### 4. `env!` (1:55)

Reads an environment variable at compile time; compilation fails if the variable is missing.

- **Example:** `let db_url = env!("DATABASE_URL");`

### 5. `option_env!` (3:00)

A safer version of `env!` that returns `None` instead of panicking if the variable is missing.

- **Example:** `let port = option_env!("PORT").unwrap_or("8080");`

### 6. `write!` / `writeln!` (3:31)

Writes formatted text to any type that implements `std::fmt::Write`, not just stdout.

- **Example:** `write!(my_buffer, "Value: {}", val).unwrap();`

### 7. `pin!` (4:20)

Pins a value to the stack to create a `Pin<&mut T>`, avoiding heap allocation.

- **Example:** `let future = pin!(async_task());`

### 8. `stringify!` (5:05)

Turns a Rust expression into its string representation.

- **Example:** `println!("{}", stringify!(x + y)); // Prints "x + y"`

### 9. `compile_error!` (6:08)

Forces a custom error message during compilation.

- **Example:** `compile_error!("This feature is not supported on this platform");`

### 10. `eprintln!` (6:58)

Prints messages specifically to `stderr`, keeping error logs separate from standard program output.

- **Example:** `eprintln!("Error: Connection failed");`
