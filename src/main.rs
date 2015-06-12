extern crate ai;

use ai::agent::Agent;
use ai::environment::Environment;
use ai::environment::CoinFlip;
use ai::random::RandomImpl;
use ai::types::{Action, Observation, Reward};

use std::rc::Rc;

fn main() {
  let mut rand = Rc::new(RandomImpl::new(5761567));

  // Setup the agent and the environment.
  let mut environment = CoinFlip::new(rand);
  let mut agent = Agent::new(environment.num_actions(), rand);

  // Let the agent loose on the environment.
  let n_cycles = 10;
  println!("Starting simulation with {} cycles", n_cycles);
  for cycle in 0..n_cycles {
    let observation = environment.observation();
    let reward = Reward(environment.reward());

    agent.update(observation, reward);
    let action = agent.act();
    assert!(action >= 0 && action < environment.num_actions());

    println!("Cycle: {}, [observation={}, {:?}], took action {}",
        cycle, observation, reward, action);

    environment.update(action);
  }
  
  // Report results.
  println!("The average reward after {} rounds is {:?}", agent.age(), agent.average_reward());
}
