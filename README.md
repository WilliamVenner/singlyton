# Singlyton

Safe, single-threaded global state in Rust.

Debug assertions are present to ensure:

* Borrow checking (see [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html))
* Thread safety (two threads cannot access the same singleton)
* Sound usage of uninitialized memory

# Example

## `Singleton`

```rust
static SINGLETON: Singleton<&'static str> = Singleton::new("Hello");
debug_assert_eq!(*SINGLETON.get(), "Hello");

SINGLETON.replace("Test");
debug_assert_eq!(*SINGLETON.get(), "Test");

*SINGLETON.get_mut() = "Test 2";
debug_assert_eq!(*SINGLETON.get(), "Test 2");
```

## `SingletonUninit`

```rust
static SINGLETON: SingletonUninit<String> = SingletonUninit::uninit();

SINGLETON.init("Hello".to_string());
debug_assert_eq!(SINGLETON.get().as_str(), "Hello");

SINGLETON.replace("Test".to_string());
debug_assert_eq!(SINGLETON.get().as_str(), "Test");

*SINGLETON.get_mut() = "Test 2".to_string();
debug_assert_eq!(SINGLETON.get().as_str(), "Test 2");
```