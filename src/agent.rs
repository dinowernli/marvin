use random::Random;
use types::Reward;

pub struct Agent<'a> {
  age: i32,
  total_reward: Reward,
  num_actions: i16,
  random: &'a mut Random,
}

impl <'a> Agent<'a> {
  pub fn new(num_actions: i16, random: &'a mut Random) -> Agent {
    Agent {
      age: 0,
      total_reward: Reward(0.0),
      num_actions: num_actions,
      random: random,
    }
  }

  pub fn age(&self) -> i32 {
    self.age
  }

  pub fn total_reward(&self) -> Reward {
    self.total_reward
  }

  /// Returns an action in [0, num_actions - 1].
  pub fn act(&mut self) -> i16 {
    // Picks a random legal action for now.
    return self.random.next_modulo(self.num_actions as i64) as i16;
  }

  pub fn update(&mut self, observation: i16, reward: Reward) {
    self.age = self.age + 1;
    self.total_reward = self.total_reward + reward;
  }

  pub fn average_reward(&self) -> Reward {
    return self.total_reward / (self.age as f64);
  }
}
