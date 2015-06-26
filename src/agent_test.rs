use agent::Agent;
use context_tree::Predictor;
use random::Random;
use types::{Observation, Reward};

#[test]
fn age() {
  let mut fake_random = FakeRandom::new();
  let fake_predictor = Box::new(FakePredictor::new());
  let mut agent = Agent::new(
      10, &mut fake_random, fake_predictor);

  assert_eq!(0, agent.age());
  agent.update(Observation(3), Reward(4.0));
  assert_eq!(1, agent.age());
}


// Fake predictor.

struct FakePredictor;

impl FakePredictor {
  fn new() -> Self {
    FakePredictor
  }
}

impl Predictor for FakePredictor {
}


// Fake random number generator.

struct FakeRandom {
  value: u64,
}

impl FakeRandom {
  fn new() -> Self {
    return FakeRandom {
      value: 4,
    };
  }
}

impl Random for FakeRandom {
  #[allow(unused_variables)]  // Ignores the limit.
  fn next_modulo(&mut self, limit: u64) -> u64 {
    return self.value;
  }
}
