use agent::Agent;
use random::Random;
use types::Reward;

#[test]
fn age() {
  let mut random = FakeRandom::new();
  let mut agent = Agent::new(10, &mut random);
  assert_eq!(0, agent.age());
  agent.update(3, Reward(4.0));
  assert_eq!(1, agent.age());
}

struct FakeRandom {
  value: i16,
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
  fn next_modulo(&mut self, limit: i64) -> i64 {
    return self.value as i64;
  }
}
