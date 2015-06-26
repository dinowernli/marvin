extern crate ai;

use ai::agent::Agent;
use ai::environment::Environment;
use ai::environment::CoinFlip;
use ai::random::RandomImpl;

// TODO(dinowernli): Replace these with command line flags.
const CONTEXT_TREE_DEPTH: usize = 4;

// Without this, cargo test warns that "main" is unused.
#[cfg_attr(test, allow(dead_code))]
fn main() {
  // Use one RNG to bootstrap the others so that we only have one
  // magic seed constant.
  let mut rand1 = RandomImpl::create(5761567);
  let mut rand2 = rand1.new_child();

  // Setup the agent and the environment.
  let mut environment = CoinFlip::new(&mut rand1);
  let mut agent = Agent::create_aixi(
      environment.num_actions(),
      &mut rand2,
      CONTEXT_TREE_DEPTH);

  // Let the agent interact with the environment.
  let n_cycles = 10;
  println!("Starting simulation with {} cycles", n_cycles);
  for cycle in 0..n_cycles {
    let action = agent.act();
    environment.update(action);

    let observation = environment.observation();
    let reward = environment.reward();
    agent.update(observation, reward);

    println!("Cycle: {}, [{:?}, {:?}, {:?}]",
        cycle, action, observation, reward);
  }
  
  // Report results.
  println!("The average reward after {} rounds is {:?}", 
      agent.age(), agent.average_reward());
}
