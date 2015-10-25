// The MIT License (MIT)
//
// Copyright (c) 2015 dinowernli
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate ai;
#[macro_use] extern crate log;

use ai::agent::Agent;
use ai::environment::Environment;
use ai::environment::coin_flip::CoinFlip;
use ai::logger::StdoutLogger;
use ai::random::RandomImpl;

use log::LogLevelFilter;

// TODO(dinowernli): Replace these with command line flags.
const CONTEXT_TREE_DEPTH: usize = 4;
const MAX_LOG_LEVEL: LogLevelFilter = LogLevelFilter::Info;

// Without this, cargo test warns that "main" is unused.
#[cfg_attr(test, allow(dead_code))]
fn main() {
  setup_logger();

  // Use one RNG to bootstrap the others so that we only have one
  // magic seed constant.
  let mut rand = RandomImpl::create(5761567);

  // Setup the agent and the environment.
  let mut environment = CoinFlip::new(&mut rand);
  let mut agent = Agent::create_aixi(
      environment.info(),
      CONTEXT_TREE_DEPTH);

  // Let the agent interact with the environment.
  let n_cycles = 10;
  info!("Starting simulation with {} cycles", n_cycles);
  for cycle in 0..n_cycles {
    let action = agent.act();
    environment.update(action);

    let observation = environment.observation();
    let reward = environment.reward();
    agent.update(observation, reward);

    info!("Cycle: {}, [{:?}, {:?}, {:?}]",
        cycle, action, observation, reward);
  }

  // Report results.
  info!("The average reward after {} rounds is {:?}",
      agent.age(), agent.average_reward());
}

// Installs a logger which handles all log macro invocations or panics.
fn setup_logger() {
  log::set_logger(|max_log_level| {
    // We're ignoring everything above the max level inside the logger anyway,
    // so here we tell the logging macros that the call can be skipped.
    max_log_level.set(MAX_LOG_LEVEL);
    Box::new(StdoutLogger::new(MAX_LOG_LEVEL.to_log_level().unwrap()))
  }).unwrap();
}
