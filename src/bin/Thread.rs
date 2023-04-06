use std::thread;
use std::time::Duration;
fn main() {
    let iterations = 10;
    let a = thread::spawn(move || {
        for i in 1..=iterations {
            println!("A: {}", i);
        }
    });
    let b = thread::spawn(move || {
        for i in 1..=iterations {
            println!("B: {}", i);
        }
    });
    a.join();
    b.join();

    let value = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });
    println!("Waiting on thread");
    match value.join() {
        Ok(val) => println!("value: {}", val),
        Err(e) => println!("error joining thread: {:?}", e),
    }
}
