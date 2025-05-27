from ctypes import *
import os

# Load the Rust library
lib = CDLL("/Users/machbluex/Desktop/lol/target/aarch64-apple-darwin/release/librust_async.dylib")

# Configure function signatures
lib.say_hello_after_delay_ffi.argtypes = [c_char_p, c_uint64]
lib.say_hello_after_delay_ffi.restype = c_void_p

lib.calculate_fibonacci_ffi.argtypes = [c_uint32]
lib.calculate_fibonacci_ffi.restype = c_uint64

lib.fetch_data_ffi.argtypes = [c_uint32]
lib.fetch_data_ffi.restype = c_void_p

lib.free_string.argtypes = [c_void_p]
lib.free_string.restype = None

def say_hello(name: str, seconds: int) -> str:
    """Call the Rust say_hello_after_delay function"""
    print(f"Python: Calling say_hello with name={name}, seconds={seconds}")
    name_bytes = name.encode('utf-8')
    print(f"Python: Encoded name bytes: {name_bytes}")
    result = lib.say_hello_after_delay_ffi(name_bytes, seconds)
    print(f"Python: Got result from FFI: {result}")
    if result:
        try:
            decoded = string_at(result).decode('utf-8')
            print(f"Python: Decoded result: {decoded}")
            return decoded
        finally:
            print("Python: Freeing string")
            lib.free_string(c_void_p(result))
    print("Python: Result was None")
    return None

def calculate_fibonacci(n: int) -> int:
    """Call the Rust calculate_fibonacci function"""
    return lib.calculate_fibonacci_ffi(n)

def fetch_data(id: int) -> str:
    """Call the Rust fetch_data function"""
    result = lib.fetch_data_ffi(id)
    if result:
        try:
            return string_at(result).decode('utf-8')
        finally:
            lib.free_string(result)
    return None

# Example usage
if __name__ == "__main__":
    # Test say_hello
    greeting = say_hello("Python", 2)
    print(f"Greeting: {greeting}")
    
    # Test fibonacci
    fib = calculate_fibonacci(10)
    print(f"Fibonacci(10) = {fib}")
    
    # Test fetch_data
    data = fetch_data(42)
    print(f"Fetched data: {data}")
    
    # Test error case
    error_data = fetch_data(0)
    print(f"Error case: {error_data}") 