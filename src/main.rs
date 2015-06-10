extern crate ai;

use ai::agent::Agent;
use ai::environment::Environment;
use ai::environment::CoinFlip;
use ai::random::Random;

fn main() {
  // TODO(dinowernli): put this behind some kind of flag.
  // test_random();

  let mut environment = CoinFlip::new();
  let mut agent = Agent::new(environment.num_actions());

  println!("Starting simulation");
  for cycle in 0..10 {
    let observation = environment.observation();
    let reward = environment.reward();
    println!("Cycle: {}, [observation={}, reward={}]", cycle, observation, reward);

    agent.update(observation, reward);
    let action = agent.act();
    assert!(action >= 0 && action < environment.num_actions());
    println!("Agent took action: {}", action);

    environment.update(action);
  }
  
  println!("Average reward after {} rounds is {}", agent.age(), agent.average_reward());
}

/// Benchmarks our random number generator by sampling it and producing a histogram.
fn test_random() {
  let mut random = Random::new(8374586759);
  let k = 10;    // Range of numbers.
  let n = 1000;  // Number of iterations.

  // Grab a total of n random numbers in [0, k-1] and build a histogram.
  let mut accum = vec![0; k];
  for _ in 0..n {
    let x = random.next_modulo(10);
    let index = x as usize;
    accum[index] = accum[index] + 1;
  }

  // Normalize the histogram by dividing by n.
  // TODO(dinowernli) Figure out how to use map.
  let mut ratios = vec![0.0; k];
  for i in 0..k {
    ratios[i] = (accum[i] as f64) / (n as f64);
  }
  println!("Ratios: {:?}", ratios);
}
