use random::Random;

pub struct Agent {
  age: i32,
  total_reward: f64,
  num_actions: i16,
  random: Box<Random>,
}

impl Agent {
  pub fn new(num_actions: i16, random: Box<Random>) -> Agent {
    Agent {
      age: 0,
      total_reward: 0.0,
      num_actions: num_actions,
      random: random,
    }
  }

  pub fn age(&self) -> i32 {
    self.age
  }

  pub fn total_reward(&self) -> f64 {
    self.total_reward
  }

  /// Returns an action in [0, num_actions - 1].
  pub fn act(&mut self) -> i16 {
    // Picks a random legal action for now.
    return (*self.random).next_modulo(self.num_actions as i64) as i16;
  }

  pub fn update(&mut self, observation: i16, reward: f64) {
    self.age = self.age + 1;
    self.total_reward += reward;
  }

  pub fn average_reward(&self) -> f64 {
    return self.total_reward / (self.age as f64);
  }
}
