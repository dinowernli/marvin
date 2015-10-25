// The MIT License (MIT)
//
// Copyright (c) 2015 dinowernli
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use explorer::{ExplorerFactory, ExplorerFactoryImpl};
use predictor::context_tree::ContextTree;
use predictor::Predictor;
use types::{Action, Observation, Reward, SingleReward};

/// Contains basic information about the environment and represents all
/// a-priori knowledge of the agent about the environment.
pub struct EnvironmentInfo {
  num_actions: i16,
  min_reward: SingleReward,
  max_reward: SingleReward,
}

impl EnvironmentInfo {
  pub fn new(
      num_actions: i16,
      min_reward: SingleReward,
      max_reward: SingleReward) -> EnvironmentInfo {
    EnvironmentInfo {
      num_actions: num_actions,
      min_reward: min_reward,
      max_reward: max_reward,
    }
  }

  pub fn num_actions(&self) -> i16 { self.num_actions }
  pub fn min_reward(&self) -> SingleReward { self.min_reward }
  pub fn max_reward(&self) -> SingleReward { self.max_reward }
}

/// Model struct for an agent which can interact with an environment.
pub struct Agent {
  age: i32,
  total_reward: Reward,
  environment_info: EnvironmentInfo,

  /// This agent's model of the environment. Used to predict
  /// (observation, reward) pairs in order to decide how to act.
  predictor: Box<Predictor>,

  /// A factory used to create an explorer whenever this agents needs to
  /// decide what its next action will be.
  explorer_factory: Box<ExplorerFactory>,
}

impl Agent {
  pub fn create_aixi(
      environment_info: EnvironmentInfo,
      context_tree_depth: usize) -> Self {
    Agent::new(
        environment_info,
        Box::new(ContextTree::create(context_tree_depth)),
        Box::new(ExplorerFactoryImpl::new()))
  }

  /// Visible for testing.
  pub fn new(
      environment_info: EnvironmentInfo,
      predictor: Box<Predictor>,
      explorer_factory: Box<ExplorerFactory>) -> Self {
    Agent {
      age: 0,
      total_reward: Reward(0.0),
      environment_info: environment_info,
      predictor: predictor,
      explorer_factory: explorer_factory,
    }
  }

  pub fn age(&self) -> i32 { self.age }

  /// Returns the total reward accumulated so far.
  pub fn total_reward(&self) -> Reward { self.total_reward }

  /// Returns the average reward accumulated so far.
  pub fn average_reward(&self) -> Reward {
    return self.total_reward / (self.age as f64);
  }

  /// Returns an action which is valid with respect to the environment, i.e.,
  /// an action within [0, environment_info.num_actions - 1].
  pub fn act(&mut self) -> Action {
    // TODO(dinowernil): Pass the entire info to the explorer.
    let num_actions = self.environment_info.num_actions;

    let mut mc_explorer = self.explorer_factory.create_monte_carlo_explorer(
        &mut *self.predictor);
    mc_explorer.explore(num_actions);
    // TODO(dinowernli): Return this result. For now, use the random explorer.

    let mut random_explorer = self.explorer_factory.create_random_explorer();
    return random_explorer.explore(num_actions);
  }

  /// Update the agent's view of the world based on a new
  /// (observation, reward) pair.
  pub fn update(&mut self, observation: Observation, reward: SingleReward) {
    // TODO(dinowernli): Inform the predictor of the new data.
    #![allow(unused_variables)]
    self.age = self.age + 1;
    self.total_reward = self.total_reward + reward;
  }
}
