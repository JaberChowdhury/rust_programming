# Rust এর Ownership, Borrowing আর Reference/Pointer বোঝা যাক 🦀

সহজ analogy দিয়ে শুরু করি — **একটা বই** এর কথা চিন্তা করো।

---

## 1. Ownership (মালিকানা)

Rust এ প্রতিটা value এর **একটাই owner** থাকে। কেউ যদি ওই value নিয়ে নেয় (assign করে অন্য variable এ), আগের owner আর সেটা use করতে পারে না।

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership move হয়ে গেল s1 থেকে s2 তে

    println!("{}", s1); // ❌ Error! s1 আর valid না
}
```

**Analogy:** ধরো তোমার একটা বই আছে। বইটা তুমি বন্ধুকে দিয়ে দিলে (না, lend না, একদম দিয়ে দিলে)। এখন বইটা তোমার কাছে নেই, বন্ধুর কাছে। তুমি চাইলেও আর পড়তে পারবা না, কারণ ownership তার কাছে চলে গেছে।

এইজন্যই উপরের code এ error আসে — `s1` আর `s2` দুইজনেই একসাথে বইটার owner হতে পারে না।

যখন owner এর scope (মানে `{ }`) শেষ হয়ে যায়, তখন Rust automatically সেই memory free করে দেয় — কোনো garbage collector লাগে না।

---

## 2. Borrowing (ধার নেওয়া) — Reference দিয়ে

পুরো ownership না নিয়ে, শুধু সাময়িকভাবে ব্যবহার করতে চাইলে **borrow** করা হয় `&` দিয়ে।

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // শুধু ধার দিলাম, ownership move হয়নি

    println!("{} এর length হলো {}", s1, len); // s1 এখনো valid! ✅
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // এখানে ফাংশন শেষ, কিন্তু s1 এর কিছু হয় না কারণ ownership আসেইনি
```

**Analogy:** এবার বইটা বন্ধুকে শুধু পড়তে দিলে (lend করলে), দিয়ে বললে "পড়ে ফেরত দিস"। ownership এখনো তোমার কাছেই, বন্ধু শুধু temporary access পেল।

---

## 3. Mutable Borrowing

Value change করতে চাইলে `&mut` দিয়ে mutable borrow করতে হয়।

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### 🔑 সবচেয়ে গুরুত্বপূর্ণ rule:

একটা value এর জন্য একসাথে থাকতে পারবে —

- **যেকোনো সংখ্যক immutable reference (`&`)**, অথবা
- **শুধুমাত্র ১টা mutable reference (`&mut`)**

দুইটা একসাথে না।

```rust
let mut s = String::from("hello");

let r1 = &s; // ✅ ঠিক আছে
let r2 = &s; // ✅ ঠিক আছে, দুইটাই immutable
println!("{} {}", r1, r2);

let r3 = &mut s; // ✅ এখন r1, r2 আর ব্যবহার হচ্ছে না, তাই ঠিক আছে
```

**কেন এই rule?** যাতে একই সময় দুইজন বইটা পড়ছে (immutable) সেটা সমস্যা না, কিন্তু একজন পড়ছে আরেকজন সেই সময় বইয়ের পাতা ছিঁড়ে ফেলছে (mutate করছে) — এইটা যেন না হয়। এভাবেই Rust compile-time এ data race আটকায়, কোনো runtime check ছাড়াই।

---

## 4. Reference vs Pointer

|                | Reference (`&T`, `&mut T`)                                 | Raw Pointer (`*const T`, `*mut T`)    |
| -------------- | ---------------------------------------------------------- | ------------------------------------- |
| Safety         | Borrow checker validate করে, সবসময় valid data নির্দেশ করে | কোনো guarantee নাই, `null` ও হতে পারে |
| ব্যবহার        | Normal (safe) Rust code এ                                  | শুধু `unsafe` block এ                 |
| Null হতে পারে? | না, কখনোই না                                               | হ্যাঁ                                 |

তোমার দৈনন্দিন Rust code এ প্রায় সবসময় **reference (`&`)** ব্যবহার হবে, raw pointer লাগে খুবই কম, মূলত FFI (C code এর সাথে যোগাযোগ) বা low-level unsafe কাজে।

---

## 5. Smart Pointer — `Box<T>` (bonus, তুমি যেহেতু rbtop বানিয়েছ, এটা জানা লাগবেই)

`Box<T>` দিয়ে data heap এ রাখা যায়, আর normal variable এর মতোই ownership rule follow করে।

```rust
fn main() {
    let b = Box::new(5); // 5 কে heap এ রাখলাম
    println!("b = {}", b);
}
```

**কেন লাগে?** যখন data এর size compile-time এ জানা যায় না (যেমন recursive struct — linked list), অথবা বড় data কে heap এ রেখে stack হালকা রাখতে চাও, তখন `Box` ব্যবহার হয়।

---

## এক লাইনে সারমর্ম

- **Ownership** → একটা মাত্র owner, scope শেষ হলে value drop হয়ে যায়
- **Borrowing (`&`)** → ownership না নিয়ে শুধু use করা
- **Mutable borrow (`&mut`)** → একটা সময়ে একটাই, আর সাথে immutable কিছু থাকতে পারবে না
- **Reference** → safe, compiler validate করে
- **Raw pointer** → unsafe, নিজের দায়িত্বে
