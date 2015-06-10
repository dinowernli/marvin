extern crate ai;

use ai::agent::Agent;
use ai::environment::Environment;
use ai::environment::CoinFlip;

fn main() {
  let mut environment = CoinFlip::new();
  let mut agent = Agent::new(environment.num_actions());

  println!("Starting simulation");
  for cycle in 0..10 {
    let observation = environment.observation();
    let reward = environment.reward();
    println!("Cycle: {}, [observation={}, reward={}]", cycle, observation, reward);

    agent.update(observation, reward);
    let action = agent.act();
    println!("Agent took action: {}", action);

    environment.update(action);
  }
  
  println!("Average reward after {} rounds is {}", agent.age(), agent.average_reward());
}
