# marvin
A general-purpose AI and training system built in Rust.

Includes an agent and a few environments. The agent applies reinforcement learning and is based on the 2010 paper "A Monte-Carlo AIXI Approximation". It uses a context tree to model the environment and does a Monte-Carlo search to determine the best action to take.

Build:
> cargo build

Run:
> cargo run
