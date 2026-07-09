# Rust Data Types — Complete Guide

Rust's type system splits into two big families: **scalar types** (single value) and **compound types** (multiple values grouped together). Everything is statically typed and checked at compile time.

## 1. Scalar Types

### Integers

| Type              | Size                                           | Range                       |
| ----------------- | ---------------------------------------------- | --------------------------- |
| `i8` / `u8`       | 8-bit                                          | -128..127 / 0..255          |
| `i16` / `u16`     | 16-bit                                         | -32,768..32,767 / 0..65,535 |
| `i32` / `u32`     | 32-bit                                         | ~-2.1B..2.1B / 0..4.3B      |
| `i64` / `u64`     | 64-bit                                         | huge range                  |
| `i128` / `u128`   | 128-bit                                        | enormous range              |
| `isize` / `usize` | pointer-sized (32 or 64-bit depending on arch) | used for indexing, sizes    |

- `i` prefix = signed (can be negative), `u` prefix = unsigned (non-negative only).
- Default integer type if unspecified: **`i32`**.
- `usize`/`isize` are special — their size matches the platform's pointer width, and Rust requires `usize` for array/vector indexing.
- Integer literals can use suffixes (`57u8`), prefixes for base (`0x` hex, `0o` octal, `0b` binary), and underscores for readability (`1_000_000`).
- Overflow: in debug builds, overflow panics; in release builds, it wraps (two's complement) unless you use explicit methods like `wrapping_add`, `checked_add`, `saturating_add`, `overflowing_add`.

```rust
let a: i32 = -42;
let b: u8 = 255;
let c = 98_222; // inferred i32
let hex = 0xff;
```

### Floating-Point

| Type  | Size   | Standard                            |
| ----- | ------ | ----------------------------------- |
| `f32` | 32-bit | single precision IEEE-754           |
| `f64` | 64-bit | double precision IEEE-754 (default) |

```rust
let x = 2.0;      // f64 by default
let y: f32 = 3.0; // explicit f32
```

### Boolean

`bool` — one byte, only two values: `true` or `false`.

```rust
let is_active: bool = true;
```

### Character

`char` — represents a **Unicode Scalar Value**, not just ASCII. It's **4 bytes**, always, and can hold any Unicode character (letters, emojis, accented characters, etc.), not just a single "byte" like C's `char`.

```rust
let letter = 'a';
let emoji = '😻';
let unicode = 'Ω';
```

Note: `char` is written with single quotes; strings use double quotes.

## 2. Compound Types

### Tuples

Fixed-length, can mix different types. Declared with parentheses.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;      // destructuring
let five_hundred = tup.0; // access by index
```

- The **unit type** `()` is a special empty tuple — represents "no meaningful value," similar to `void`. Functions with no return value implicitly return `()`.

### Arrays

Fixed-length, all elements **same type**, stored on the stack.

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let same = [3; 5]; // [3, 3, 3, 3, 3]
let first = arr[0];
```

- Length is part of the type (`[i32; 5]` ≠ `[i32; 10]`).
- Bounds-checked at runtime — out-of-bounds access panics rather than causing undefined behavior.
- Use arrays when the length is known and fixed (e.g., days of the week). For growable collections, use `Vec<T>`.

## 3. Common Standard Library / Collection Types

These aren't "primitive" but are essential to know:

### String types

- **`String`** — growable, heap-allocated, owned, UTF-8 encoded string.
- **`&str`** (string slice) — an immutable view/reference into string data (could be part of a `String` or a string literal). String literals in code are `&'static str`.

```rust
let s1: String = String::from("hello");
let s2: &str = "world"; // string literal
```

### Vec<T>

Growable array on the heap.

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
let v2 = vec![1, 2, 3];
```

### Other key collections

- `HashMap<K, V>` — key-value store.
- `HashSet<T>` — unique unordered set.
- `VecDeque<T>` — double-ended queue.
- `BTreeMap<K, V>` / `BTreeSet<T>` — ordered map/set.

## 4. Custom / Compound-Defining Types

### Structs

Group related named fields into one type.

```rust
struct User {
    username: String,
    active: bool,
}

// Tuple struct
struct Point(i32, i32, i32);

// Unit-like struct (no fields)
struct Marker;
```

### Enums

A type that can be one of several defined variants — very powerful in Rust, can hold data per variant.

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Point,
}
```

### Option<T> and Result<T, E>

Rust has **no null**. Instead:

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

These are used everywhere for handling "maybe absent" values and error handling, forcing explicit handling at compile time.

## 5. Pointer / Reference Types

- **References**: `&T` (immutable borrow), `&mut T` (mutable borrow) — don't own the data.
- **Smart pointers**:
  - `Box<T>` — heap allocation, single owner.
  - `Rc<T>` — reference-counted, single-threaded shared ownership.
  - `Arc<T>` — atomic reference-counted, thread-safe shared ownership.
  - `RefCell<T>` / `Cell<T>` — interior mutability.

```rust
let b: Box<i32> = Box::new(5);
let r: &i32 = &b;
```

## 6. Type Inference & Annotations

Rust infers types most of the time, but you can (and sometimes must) annotate explicitly:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

---

**Quick mental model:**

- Need a single number/bool/char? → **scalar types**.
- Need a fixed group of mixed types? → **tuple**.
- Need a fixed-size list of the same type? → **array**.
- Need a growable list? → **Vec**.
- Need text? → **String** (owned) or **&str** (borrowed).
- Need custom structured data? → **struct** or **enum**.
- Need "maybe nothing" or "maybe error"? → **Option** / **Result**.
  Great — let's go deeper into both **ownership/borrowing** and **memory layout (stack vs heap)**, since they explain _why_ Rust's types behave the way they do.

## Stack vs Heap — Where Types Live

### Stack

Fast, fixed-size, LIFO (last-in-first-out). Rust puts a value on the stack when its **size is known at compile time**.

Stored on the stack:

- All scalar types (`i32`, `f64`, `bool`, `char`)
- Arrays (`[i32; 5]`) — because length is part of the type
- Tuples of stack types
- Structs made entirely of stack types

```rust
let x = 5;          // stack
let arr = [1, 2, 3]; // stack
```

### Heap

Used when size is **unknown at compile time** or can **grow/shrink**. Slower to allocate, accessed via pointer.

Stored on the heap (with a stack-based pointer/metadata pointing to it):

- `String` — stack holds a pointer, length, and capacity; actual char data is on the heap.
- `Vec<T>` — same pattern: pointer + length + capacity on stack, data on heap.
- `Box<T>` — explicitly puts _any_ value on the heap.

```rust
let s = String::from("hello");
// stack: { ptr, len: 5, capacity: 5 }
// heap:  h e l l o
```

This is why `String` can grow (`push_str`) but a string literal `&str` and arrays can't — their size is fixed.

## Ownership — The Core Rule

Every value in Rust has exactly **one owner**. When the owner goes out of scope, the value is dropped (freed).

```rust
{
    let s = String::from("hello"); // s owns this data
} // s goes out of scope here, memory freed automatically
```

### Move semantics

For heap-backed types (`String`, `Vec`, etc.), assigning to another variable **moves** ownership — it does NOT copy the heap data. The old variable becomes invalid.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is MOVED into s2
// println!("{}", s1); // ERROR: s1 no longer valid
```

This prevents double-free bugs — only one owner is ever responsible for freeing the heap memory.

### Copy types

Scalar types (`i32`, `f64`, `bool`, `char`) and tuples/arrays of them implement the `Copy` trait — assignment **copies** the value instead of moving it, since stack copying is cheap and there's no heap data to worry about.

```rust
let x = 5;
let y = x; // x is COPIED, both x and y are valid
println!("{} {}", x, y); // fine!
```

**Rule of thumb:** if a type is entirely on the stack and cheap to duplicate → `Copy`. If it manages heap memory → move-only (unless you `.clone()` explicitly).

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // explicit deep copy of heap data
println!("{} {}", s1, s2); // both valid
```

## Borrowing — References Without Taking Ownership

Instead of moving/copying, you can **borrow** a value with `&`.

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but doesn't own the data, so nothing is dropped

let s1 = String::from("hello");
let len = calculate_length(&s1); // borrow, s1 still valid after
```

### The Borrowing Rules (enforced at compile time)

1. You can have **any number of immutable references** (`&T`) at once, **OR**
2. Exactly **one mutable reference** (`&mut T`) — but not both at the same time.
3. References must always be **valid** (no dangling references).

```rust
let mut s = String::from("hello");

let r1 = &s;     // OK
let r2 = &s;     // OK, multiple immutable borrows fine
// let r3 = &mut s; // ERROR: can't borrow as mutable while immutable borrows exist

println!("{} {}", r1, r2);
```

```rust
let mut s = String::from("hello");
let r1 = &mut s;
r1.push_str(" world"); // OK
// let r2 = &s; // ERROR here if r1 still in use — no immutable ref while mutable exists
```

This is what prevents **data races** at compile time — no runtime cost, no garbage collector needed.

## Slices — Borrowing Part of a Collection

A slice is a reference to a contiguous portion of a collection, without ownership.

```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];
let world: &str = &s[6..11];

let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..3]; // [2, 3]
```

`&str` is actually always a slice — it's `&[u8]`-like data guaranteed to be valid UTF-8, with a pointer + length, no capacity (since it's not growable).

## Lifetimes — Ensuring References Don't Outlive Their Data

Rust's borrow checker uses **lifetimes** to ensure a reference never outlives the value it points to.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` here says: the returned reference lives at most as long as the shorter of `x` and `y`'s lifetimes. Most of the time lifetimes are inferred (lifetime elision), and you only write them explicitly when the compiler can't figure it out on its own — usually in function signatures involving references.

## Putting It Together — A Mental Model

| Concept         | Analogy                                                      |
| --------------- | ------------------------------------------------------------ |
| Owner           | The one person responsible for a book                        |
| Move            | Handing the book to someone else — you no longer have it     |
| Clone           | Photocopying the book — now two independent copies exist     |
| Copy (scalar)   | Like sharing a small sticky note — trivial to duplicate      |
| `&T` borrow     | Letting someone read the book without taking it              |
| `&mut T` borrow | Letting exactly one person write in the book, temporarily    |
| Lifetime        | A guarantee the book still exists while someone's reading it |

This ownership/borrowing system is _the_ defining feature of Rust — it's what lets Rust guarantee memory safety **without a garbage collector**, all checked at compile time with zero runtime cost.

Want me to walk through a concrete example — like writing a small program that shows ownership errors and how to fix them, or dig into smart pointers (`Box`, `Rc`, `Arc`, `RefCell`) next?
