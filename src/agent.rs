pub struct Agent {
  age: i32,
  total_reward: f64,
}

impl Agent {
  pub fn new() -> Agent {
    Agent {
      age: 0,
      total_reward: 0.0,
    }
  }

  pub fn age(&self) -> i32 {
    self.age
  }

  pub fn total_reward(&self) -> f64 {
    self.total_reward
  }

  pub fn act(&mut self) -> i16 {
    return 4;  // Constant action for now.
  }

  pub fn update(&mut self, observation: i16, reward: f64) {
    self.age = self.age + 1;
    self.total_reward += reward;
  }

  pub fn average_reward(&self) -> f64 {
    return self.total_reward / (self.age as f64);
  }
}
