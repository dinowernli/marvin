extern crate ai;

use ai::agent::Agent;
use ai::environment::Environment;
use ai::environment::CoinFlip;

fn main() {
  let mut agent = Agent::new();

  println!("Starting simulation");
  for _ in 0..10 {
    let action = agent.act();
    println!("Got action {}", action);
    agent.update(2, 3.5);
  }
  
  println!("Average reward after {} rounds is {}", agent.age(), agent.average_reward());
}
