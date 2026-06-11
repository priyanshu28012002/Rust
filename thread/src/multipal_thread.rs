use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{thread, time::Duration};

fn main() {
    let running = Arc::new(AtomicBool::new(true));

    // Ctrl+C handler
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl+C received");
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    let num_threads = 4; // change this to N
    let mut handles = Vec::new();

    for i in 0..num_threads {
        let r = running.clone();

        let handle = thread::spawn(move || {
            while r.load(Ordering::SeqCst) {
                println!("Worker {} running...", i);
                thread::sleep(Duration::from_millis(500));
            }
            println!("Worker {} stopping...", i);
        });

        handles.push(handle);
    }

    // Main loop
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(200));
    }

    // Join ALL threads (important)
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All workers stopped. Clean exit.");
}