mod lib;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use lib::event::CrossProcessAsyncEvent;

fn main() {
    // Atomic boolean for communication between the system callback call and our process.
    let shared1 = Arc::new(AtomicBool::new(false));

    let mut counter = 0;
    while counter < 3 {
        // Create out primitive
        let mut event = CrossProcessAsyncEvent::try_create("some-random-event")
            .expect("Failed to create event.");
        // Callback captures the atomic boolean
        let shared2 = shared1.clone();
        let callback = move || {
            println!("Hello from callback!");
            shared2.store(true, Ordering::Relaxed);
        };

        let registered = event.register_callback(callback);
        println!("Callback registraction status: {}", registered);
        println!("Waiting for waker.");
        // I used a busy loop for testing, this is where an `await` call would be made
        while !shared1.load(Ordering::Relaxed) {
            std::hint::spin_loop()
        }
        shared1.store(false, Ordering::Relaxed);
        counter += 1;
        println!("Received {} wakes.", counter);
    }
    println!("Terminating.");
}
