use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shared state using Arc and Mutex
    let shared_state = Arc::new(Mutex::new(0));

    // Create a vector to hold the handles of the spawned threads
    let mut handles = vec![];

    for _ in 0..4 {
        // Assume we want to run 4 threads
        // Clone the Arc to have another reference to the shared state
        let state = Arc::clone(&shared_state);

        // Spawn a new thread
        let handle = thread::spawn(move || {
            println!("thread");
            // Specify the launch options for the browser
            let options = LaunchOptionsBuilder::default().build().unwrap();
            // Launch a new browser instance
            let browser = Browser::new(options).expect("Failed to launch browser");

            // Create a new tab
            let tab = browser.new_tab().expect("Failed to create a tab");

            // Navigate to a web page
            tab.navigate_to("http://example.com")
                .expect("Failed to navigate");

            // Perform web scraping or interactions here
            // ...
            // Modify shared state
            let mut num = state.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final state
    let final_count = shared_state.lock().unwrap();
    println!("Final count is {}", *final_count);

    Ok(())
}
