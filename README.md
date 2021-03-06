# Singlyton

Safe, single-threaded global state in Rust.

Debug assertions are present to ensure:

* Borrow checking (see [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html))
* Thread safety (two threads cannot access the same singleton)
* Sound usage of uninitialized memory

# Why?

Single-threaded global state is a bit of a boogeyman in Rust:

* [`static mut`](https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics) is heavily discouraged due to its easy ability to cause UB through [aliasing](https://doc.rust-lang.org/nomicon/aliasing.html).
* [Thread locals](https://doc.rust-lang.org/std/thread/struct.LocalKey.html) can be slow for performance critical contexts, are nonsense to use in a single-threaded environment, and may not be available on all platforms
* Working around Rust's thread-safety mechanisms in single-threaded contexts can be ugly, annoying and unnecessary

# Usage

First, add `singlyton` as a dependency of your project in your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) file:

```toml
[dependencies]
singlyton = "*"
```

## `Singleton`

```rust
use singlyton::Singleton;

static SINGLETON: Singleton<&'static str> = Singleton::new("Hello");
debug_assert_eq!(*SINGLETON.get(), "Hello");

SINGLETON.replace("Test");
debug_assert_eq!(*SINGLETON.get(), "Test");

*SINGLETON.get_mut() = "Test 2";
debug_assert_eq!(*SINGLETON.get(), "Test 2");
```

## `SingletonUninit`

```rust
use singlyton::SingletonUninit;

static SINGLETON: SingletonUninit<String> = SingletonUninit::uninit();

SINGLETON.init("Hello".to_string());
debug_assert_eq!(SINGLETON.get().as_str(), "Hello");

SINGLETON.replace("Test".to_string());
debug_assert_eq!(SINGLETON.get().as_str(), "Test");

*SINGLETON.get_mut() = "Test 2".to_string();
debug_assert_eq!(SINGLETON.get().as_str(), "Test 2");
```