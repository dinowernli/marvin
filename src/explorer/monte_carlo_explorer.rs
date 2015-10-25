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

use explorer::Explorer;
use predictor::Predictor;
use random::Random;
use types::{Action, Percept, Reward};

use std::collections::HashMap;

pub struct MonteCarloExplorer<'a> {
  predictor: &'a mut Predictor,
  random: Box<Random>,
}

impl <'a> MonteCarloExplorer<'a> {
  pub fn new(predictor: &'a mut Predictor, random: Box<Random>)
      -> MonteCarloExplorer<'a> {
    MonteCarloExplorer {
      predictor: predictor,
      random: random,
    }
  }
}

impl <'a> Explorer for MonteCarloExplorer<'a> {
  fn explore(&mut self, num_actions: i16) -> Action {
    #![allow(unused_variables)]
    // TODO(dinowernli): Use self.predictor to find the best action.

    // Create the root of the tree, always an action node.
    let mut tree = ActionNode::new(num_actions);

    return Action(0);
  }
}

/// Represents a node in the exploration search tree. Each node represents
/// either an action or a percept and a path through the tree represents a
/// sequence of events, i.e., a possible future.
trait Node {
  /// Returns an estimate of the mean reward taken over all hypothetical
  /// futures which pass through this node.
  fn mean_reward(&self) -> Reward;

  /// Samples one path through this node, looking at most horizon steps into
  /// the future.
  fn sample(&mut self, horizon: usize) -> Reward;
}

/// A node which represents a possible action.
struct ActionNode {
  visits: u64,
  num_actions: i16,
  mean_reward: Reward,
  children: Box<HashMap<Percept, Box<ChanceNode>>>,
}

impl ActionNode {
  fn new(num_actions: i16) -> ActionNode {
    ActionNode {
      visits: 0,
      num_actions: num_actions,
      mean_reward: Reward(0.0),
      children: Box::new(HashMap::new()),
    }
  }

  /// Returns a mutable reference to the specified child. Lazily creates the
  /// child if it is not present.
  fn mut_child(&mut self, percept: Percept) -> &mut ChanceNode {
    if !self.children.contains_key(&percept) {
      let num_actions = self.num_actions;
      self.children.insert(percept, Box::new(ChanceNode::new(num_actions)));
    }
    return &mut **self.children.get_mut(&percept).unwrap();
  }

  /// Implements the UCB heuristic to trade off exploring unknown actions
  /// versus exploiting actions assumed to have a high payoff. Returns an
  /// action to explore next.
  fn select_explore_exploit(&self, remaining_horizon: usize)
      -> Action {
    // TODO(dinowernli): Plumb more environment information into here in order
    // to evalutae UCB (reward range, etc).
  }
}

impl Node for ActionNode {
  fn mean_reward(&self) -> Reward { self.mean_reward }

  fn sample(&mut self, horizon: usize) -> Reward {
    if horizon == 0 {
      return Reward(0.0);  // Chose as the additive neutral element.
    }
    // TODO(dinowernli): Implement.
    return self.sample(horizon - 1);
  }
}

/// A node which represents a possible reaction of the environment.
struct ChanceNode {
  visits: u64,
  num_actions: i16,
  mean_reward: Reward,
  children: Box<HashMap<Action, Box<ActionNode>>>,
}

impl ChanceNode {
  fn new(num_actions: i16) -> ChanceNode {
    ChanceNode {
      visits: 0,
      num_actions: num_actions,
      mean_reward: Reward(0.0),
      children: Box::new(HashMap::new()),
    }
  }

  /// Returns a mutable reference to the specified child. Lazily creates the
  /// child if it is not present.
  fn mut_child(&mut self, action: Action) -> &mut ActionNode {
    if !self.children.contains_key(&action) {
      let num_actions = self.num_actions;
      self.children.insert(action, Box::new(ActionNode::new(num_actions)));
    }
    return &mut **self.children.get_mut(&action).unwrap();
  }
}

impl Node for ChanceNode {
  fn mean_reward(&self) -> Reward { self.mean_reward }

  fn sample(&mut self, horizon: usize) -> Reward {
    if horizon == 0 {
      return Reward(0.0);  // Chose as the additive neutral element.
    }
    // TODO(dinowernli): Implement.
    return self.sample(horizon - 1);
  }
}

