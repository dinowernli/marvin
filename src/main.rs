extern crate ai;
extern crate rand;

use ai::agent::Agent;
use ai::environment::Environment;
use ai::environment::CoinFlip;
use ai::random::RandomImpl;
use ai::types::Reward;

use rand::{Rng, SeedableRng, StdRng};

// Without this, cargo test warns that "main" is unused.
#[cfg_attr(test, allow(dead_code))]
fn main() {
  playground();

  // Use one RNG to bootstrap the others so that we only have one magic seed constant.
  let mut rand1 = Box::new(RandomImpl::new(5761567));
  let rand2 = Box::new(rand1.new_child());

  // Setup the agent and the environment.
  let mut environment = CoinFlip::new(rand1);
  let mut agent = Agent::new(environment.num_actions(), rand2);

  // Let the agent interact with the environment.
  let n_cycles = 10;
  println!("Starting simulation with {} cycles", n_cycles);
  for cycle in 0..n_cycles {
    let action = agent.act();
    assert!(action >= 0 && action < environment.num_actions());
    environment.update(action);

    let observation = environment.observation();
    let reward = Reward(environment.reward());
    agent.update(observation, reward);

    println!("Cycle: {}, [action={}, observation={}, {:?}]",
        cycle, action, observation, reward);
  }
  
  // Report results.
  println!("The average reward after {} rounds is {:?}", agent.age(), agent.average_reward());
}

/// Testbed for experimentation.
fn playground() {
  let seed: &[_] = &[1896768];
  let mut rng: StdRng = SeedableRng::from_seed(seed);
  for _ in 0..5 {
    println!("Rng: {}", rng.gen::<u64>() % 17);
  }
}
