use std::thread;
use std::time::Duration;
fn main() {
    closuresWithThreads();
}

// Closures with Threads
fn closuresWithThreads() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move|| {
        println!("Here's a vector: {v:?}");
    });
    drop(v);
    handle.join().unwrap();
}

fn Spwan_thread_wait_to_finish() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_secs(1));
    }
    handle.join().unwrap();
}

fn Spwan_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
