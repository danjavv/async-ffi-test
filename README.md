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
## General rule

If you have a function in Rust - 
```rust
pub async fn run_compare_ge<T, R>(...) -> Result<BinaryShare, ProtocolError>
```
the synchronous, FFI-safe wrapper should be written in src/ffi.rs:
```rust
#[no_mangle]
pub extern "C" fn compare_ge_ffi(/* C-compatible args */) -> /* C-compatible return */ {
    // Convert C args to Rust types
    // Call run_compare_ge using a runtime if async
    // Convert result to C-compatible return
}
```
and then should be imported in lib.rs as -
```rust
pub mod ffi;
```
If the Rust function is returning more than one value at once - 
```rust
let (state_cb_r1, msg1) = abt_create_msg1(&session_id, &ot_seeds_cb, p, eta_i, eta_m, rng);
```
create a C-compatible struct above the function ```rust pub extern "C" fn ffi_abt_create_msg1 ``` like this - 
```rust
#[repr(C)]
pub struct FFI_AbtCreateMsg1Result {
    state_cb_r1_ptr: *mut u8,
    state_cb_r1_size: usize,
    msg1_ptr: *mut u8,
    msg1_size: usize,
}
use rand::rngs::OsRng;
```
and define it accordingly in the final python file -
```python
class FFI_AbtCreateMsg1Result(ctypes.Structure):
    _fields_ = [
        ("state_cb_r1_ptr", ctypes.POINTER(ctypes.c_ubyte)),
        ("state_cb_r1_size", ctypes.c_size_t),
        ("msg1_ptr", ctypes.POINTER(ctypes.c_ubyte)),
        ("msg1_size", ctypes.c_size_t),
    ]

rust_lib.ffi_abt_create_msg1.argtypes = [
    ctypes.POINTER(ctypes.c_ubyte), ctypes.c_size_t,  # session_id
    ctypes.POINTER(ctypes.c_ubyte), ctypes.c_size_t,  # ot_seeds_cb
    ctypes.c_void_p,  # rng_ptr
]
rust_lib.ffi_abt_create_msg1.restype = FFI_AbtCreateMsg1Result
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
    This will produce a shared library with extension .dylib (on macOS), .so (on Linux), .dll (on Windows) etc.  
    This library is getting used in the rust_async.py file

4. **Check your Python architecture**
   ```bash
   python3 -c "import platform; print(platform.machine())"
   ```
5. **Run the python file rust_async.py with compatible architecture python :**
   ```bash
   python3.13 rust_async.py
   ```
