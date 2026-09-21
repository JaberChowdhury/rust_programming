**Simplest way (single line, single value):**

```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    println!("{}", n);
}
```

**Multiple values on one line** (the `cin >> a >> b >> c` equivalent):

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let vals: Vec<i32> = input
    .trim()
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect();
let (a, b, c) = (vals[0], vals[1], vals[2]);
```

**CP-style fast I/O** (what you actually want for contests — reads whole stdin at once, tokenizes lazily, no line-by-line overhead):

```rust
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut out = String::new();

    let n: usize = it.next().unwrap().parse().unwrap();
    let a: Vec<i64> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();

    out.push_str(&format!("{}\n", a.iter().sum::<i64>()));
    print!("{}", out);
    io::stdout().flush().unwrap();
}
```

**Case 1: n values on one line** (e.g. `5\n1 2 3 4 5`)

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let vals: Vec<i32> = input
    .trim()
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect();
```

`split_whitespace()` + `.collect()` is your `for(int i=0;i<n;i++) cin >> a[i];` in one line.

**Case 2: n values across n separate lines** (e.g. n queries, one per line)

```rust
let mut lines = io::stdin().lines();
let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
let vals: Vec<i32> = (0..n)
    .map(|_| lines.next().unwrap().unwrap().trim().parse().unwrap())
    .collect();
```

`stdin().lines()` gives you an iterator over lines (each is `io::Result<String>`), so `.next().unwrap().unwrap()` pulls one line — first `unwrap()` for the IO result, second because `read`-style errors are rare in CP so you just unwrap through them.
