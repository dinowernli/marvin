use context_tree::{ContextTree, Predictor};
use random::Random;
use types::{Action, Observation, Reward};

/// Model struct for an agent which can interact with an environment.
pub struct Agent<'a> {
  age: i32,
  total_reward: Reward,
  num_actions: i16,
  random: &'a mut Random,

  /// This agent's model of the environment. Used to predict
  /// (observation, reward) pairs in order to decide how to act.
  predictor: Box<Predictor>,
}

impl <'a> Agent<'a> {
  pub fn create_aixi(
      num_actions: i16,
      random: &'a mut Random,
      context_tree_depth: usize) -> Self {
    Agent::new(
        num_actions,
        random,
        Box::new(ContextTree::create(context_tree_depth)))
  }

  /// Visible for testing.
  pub fn new(
      num_actions: i16,
      random: &'a mut Random,
      predictor: Box<Predictor>) -> Self {
    Agent {
      age: 0,
      total_reward: Reward(0.0),
      num_actions: num_actions,
      random: random,
      predictor: predictor,
    }
  }

  pub fn age(&self) -> i32 {
    self.age
  }

  /// Returns the total reward accumulated so far.
  pub fn total_reward(&self) -> Reward {
    self.total_reward
  }

  /// Returns the average reward accumulated so far.
  pub fn average_reward(&self) -> Reward {
    return self.total_reward / (self.age as f64);
  }

  /// Returns an action in [0, num_actions - 1].
  pub fn act(&mut self) -> Action {
    // Picks a random legal action for now.
    return Action(self.random.next_modulo(self.num_actions as u64) as i16);
  }

  /// Update the agent's view of the world based on a new
  /// (observation, reward) pair.
  pub fn update(&mut self, observation: Observation, reward: Reward) {
    self.age = self.age + 1;
    self.total_reward = self.total_reward + reward;
  }
}
