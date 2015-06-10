extern crate ai;

use ai::agent::Agent;
use ai::environment::Environment;
use ai::environment::CoinFlip;
use ai::random::Random;

fn main() {
  // test_random();

  // Use one RNG to bootstrap the others so that we only have one magic seed constant.
  let mut rand1 = Box::new(Random::new(5761567));
  let mut rand2 = Box::new(rand1.new_child());

  // Setup the agent and the environment.
  let mut environment = CoinFlip::new(rand1);
  let mut agent = Agent::new(environment.num_actions(), rand2);

  // Let the agent loose on the environment.
  let n_cycles = 50;
  println!("Starting simulation with {} cycles", n_cycles);
  for cycle in 0..n_cycles {
    let observation = environment.observation();
    let reward = environment.reward();

    agent.update(observation, reward);
    let action = agent.act();
    assert!(action >= 0 && action < environment.num_actions());

    println!("Cycle: {}, [observation={}, reward={}], took action {}", 
        cycle, observation, reward, action);

    environment.update(action);
  }
  
  // Report results.
  println!("The average reward after {} rounds is {}", agent.age(), agent.average_reward());
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
