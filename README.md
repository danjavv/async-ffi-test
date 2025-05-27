# Python wrappers for async Rust functions

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
3. **Check your Python architecture**
   ```bash
   python3 -c "import platform; print(platform.machine())"
   ```
4. **Run the python file rust_async.py with compatible architecture python :**
   ```bash
   python3.13 rust_async.py
   ```
