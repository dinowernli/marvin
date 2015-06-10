use agent::Agent;
use random::Random;

#[test]
fn age() {
  let mut agent = Agent::new(10, Box::new(FakeRandom::new()));
  assert_eq!(0, agent.age());
  agent.update(3, 4.0);
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
  fn next_modulo(&mut self, limit: i64) -> i64 {
    return self.value as i64;
  }
}
