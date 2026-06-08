use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{thread, time::Duration};

fn main() {
    let running = Arc::new(AtomicBool::new(true));

    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl+C received");
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    let r = running.clone();
    let handle = thread::spawn(move || {
        while r.load(Ordering::SeqCst) {
            println!("Worker running...");
            thread::sleep(Duration::from_millis(500));
        }
        println!("Worker stopping...");
    });

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(200));
    }

    handle.join().unwrap();
    println!("Clean exit");
}