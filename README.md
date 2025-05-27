# Python wrappers for async Rust functions

lib.rs contains three async Rust functions :- 

1. pub async fn say_hello_after_delay // Simple async function that waits and returns a message
2. pub async fn calculate_fibonacci // Async function that performs some computation
3. pub async fn fetch_data // Async function with error handling

ffi.rs contains python ffi for each of these functions

For example :- 

```rust
// FFI wrapper for calculate_fibonacci
#[no_mangle]
pub extern "C" fn calculate_fibonacci_ffi(n: u32) -> u64 {
    let rt = Runtime::new().unwrap();
    rt.block_on(crate::calculate_fibonacci(n))
}
```

## How to run

1. **Clone the repository:**
   ```bash
   git clone <your-repo-url>
   ```
2. **Build the lbirary according to the underlying system architecture:**
   For M1 macOS :-
   ```bash
   cargo build --release --target aarch64-apple-darwin
   ```
    This will produce a shared library with extension .dylib (on macOS), .so (on Linux), etc.

    This library is getting used in the rust_async.py file

4. **Check your Python architecture**
   ```bash
   python3 -c "import platform; print(platform.machine())"
   ```
5. **Run the python file rust_async.py with compatible architecture python :**
   ```bash
   python3.13 rust_async.py
   ```
