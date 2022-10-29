mod lib;

use std::sync::atomic::{AtomicBool, Ordering};

use lib::event::CrossProcessAsyncEvent;

fn main() {
    // Atomic boolean for communication between the system callback call and our process.
    let shared = AtomicBool::new(false);
    // Create out primitive
    let mut event =
        CrossProcessAsyncEvent::try_create("some-random-event").expect("Failed to create event.");
    // Callback captures the atomic boolean
    let callback = || {
        println!("Yoo!");
        shared.store(true, Ordering::Relaxed);
    };

    let registered = event.register_callback(callback);
    println!("Callback registraction status: {}", registered);

    let mut counter = 0;
    while counter < 3 {
        println!("Waiting for waker.");
        // I used a busy loop for testing, this is where an `await` call would be made
        while !shared.load(Ordering::Relaxed) {
            std::hint::spin_loop()
        }
        counter += 1;
        println!("Received {} wakes.", counter);
    }
    println!("Terminating.");
}
