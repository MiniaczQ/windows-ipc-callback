# :rocket: Windows :rocket: async :rocket: IPC :rocket: experiment :rocket:

To achieve true async we need the ability to remotely call a procedure on another process.

The point of this experiment was to fabricate a basis for this on Windows platform.

# How to run :running:

To try the example yourself run:

`cargo run --bin sleeper`

`cargo run --bin waker`

The intended behavior is for `sleeper` to print a console message for each `waker` ran.

The `sleeper` will should terminate after few prints.

# Other

The code is **very** unsafe.

The code is intentionaly over-documented.

The code **does not perform any cleanup**.
It's only good for 1-time use for the duration of the entire process.

I wanted to create a flexible solution to run multiple tests.
The code can be adapted for something more specific.

To sum it up, this is an experiment unfit for any practical usage. (like Haskell :wink:)

# Kudos

Big thanks to

- kpreid#9810
- T-Dark#9470

from [Rust discord](https://discord.gg/rust-lang-community).

This experiment succeeded because of Your help! :)
