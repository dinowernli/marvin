use agent::Agent;
use random::Random;
use types::{Observation, Reward};

#[test]
fn age() {
  let mut random = FakeRandom::new();
  let mut agent = Agent::new(10, &mut random);
  assert_eq!(0, agent.age());
  agent.update(Observation(3), Reward(4.0));
  assert_eq!(1, agent.age());
}

struct FakeRandom {
  value: u64,
}

impl FakeRandom {
  fn new() -> FakeRandom {
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
