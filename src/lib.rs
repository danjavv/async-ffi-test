use std::time::Duration;
use tokio::time::sleep;

pub mod ffi;

// Simple async function that waits and returns a message
pub async fn say_hello_after_delay(name: &str, seconds: u64) -> String {
    println!("Starting to wait for {} seconds...", seconds);
    
    // Async sleep - doesn't block the thread
    sleep(Duration::from_secs(seconds)).await;
    
    format!("Hello, {}! I waited {} seconds to say this.", name, seconds)
}

// Async function that performs some computation
pub async fn calculate_fibonacci(n: u32) -> u64 {
    // Simulate some async work by yielding control
    tokio::task::yield_now().await;
    
    if n <= 1 {
        return n as u64;
    }
    
    let mut a = 0u64;
    let mut b = 1u64;
    
    for _ in 2..=n {
        // Yield control occasionally during computation
        if n % 5 == 0 {
            tokio::task::yield_now().await;
        }
        
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

// Async function with error handling
pub async fn fetch_data(id: u32) -> Result<String, &'static str> {
    // Simulate network delay
    sleep(Duration::from_millis(100)).await;
    
    if id == 0 {
        Err("Invalid ID: cannot be zero")
    } else if id > 1000 {
        Err("Invalid ID: too large")
    } else {
        Ok(format!("Data for ID: {}", id))
    }
}
