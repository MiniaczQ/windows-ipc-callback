mod lib;

use lib::event::CrossProcessAsyncEvent;

fn main() {
    // We access an existing `Event` and wake it.
    let event =
        CrossProcessAsyncEvent::try_open("some-random-event").expect("Failed to open event.");
    event.wake();
}
