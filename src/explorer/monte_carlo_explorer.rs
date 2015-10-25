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

use agent::EnvironmentInfo;
use explorer::Explorer;
use predictor::Predictor;
use random::Random;
use types::{Action, Percept, Reward};

use std::collections::{HashMap, HashSet};

/// The number of visits we require to child action nodes before starting to
/// trade off explore and exploit.
const MIN_VISITS: usize = 1;

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
  fn explore(&mut self, environment_info: EnvironmentInfo) -> Action {
    #![allow(unused_variables)]
    // TODO(dinowernli): Use self.predictor to find the best action.

    // Create the root of the tree, always an action node.
    let mut tree = ActionNode::new(environment_info);

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

  /// Returns the number of times this node has been visited.
  fn visits(&self) -> usize;

  /// Returns whether or not this node requires a visit, i.e., has been visted
  /// less than the number of times considered minimal. Once a node no longer
  /// requires visits, then we start trading off explore vs. exploit.
  fn requires_visit(&self) -> bool { self.visits() < MIN_VISITS }
}

/// A node where the environment reacts.
struct ChanceNode {
  visits: usize,
  environment_info: EnvironmentInfo,
  mean_reward: Reward,
  children: Box<HashMap<Percept, Box<ActionNode>>>,
}

impl ChanceNode {
  fn new(environment_info: EnvironmentInfo) -> ChanceNode {
    ChanceNode {
      visits: 0,
      environment_info: environment_info,
      mean_reward: Reward(0.0),
      children: Box::new(HashMap::new()),
    }
  }

  /// Returns a mutable reference to the specified child. Lazily creates the
  /// child if it is not present.
  fn mut_child(&mut self, percept: Percept) -> &mut ActionNode {
    if !self.children.contains_key(&percept) {
      let info = self.environment_info;
      self.children.insert(percept, Box::new(ActionNode::new(info)));
    }
    return &mut **self.children.get_mut(&percept).unwrap();
  }
}

impl Node for ChanceNode {
  fn mean_reward(&self) -> Reward { self.mean_reward }
  fn visits(&self) -> usize { self.visits }

  fn sample(&mut self, horizon: usize) -> Reward {
    if horizon == 0 {
      return Reward(0.0);  // Chose as the additive neutral element.
    }
    // TODO(dinowernli): Implement.
    return self.sample(horizon - 1);
  }
}

/// A node where an action needs to be taken.
struct ActionNode {
  visits: usize,
  environment_info: EnvironmentInfo,
  mean_reward: Reward,
  children: Box<HashMap<Action, Box<ChanceNode>>>,
}

impl ActionNode {
  fn new(environment_info: EnvironmentInfo) -> ActionNode {
    ActionNode {
      visits: 0,
      environment_info: environment_info,
      mean_reward: Reward(0.0),
      children: Box::new(HashMap::new()),
    }
  }

  /// Implements the UCB heuristic to trade off exploring unknown actions
  /// versus exploiting actions assumed to have a high payoff. Returns an
  /// action to explore next.
  fn select_explore_exploit(&mut self, remaining_horizon: usize)
      -> Action {
    let mut require_visit: HashSet<Action> = HashSet::new();

    // Go through all possible actions and check the stat of the child nodes.
    for a in 0..self.environment_info.num_actions() - 1 {
      let action = Action(a);
      let child = self.mut_child(Action(a));
      if child.requires_visit() {
        require_visit.insert(action);
      }

      // TODO(dinowernli): Compute the UCB coefficient.
    }

    // If any children require visits, pick one uniformly at random.
    // TODO(dinowernli): Implement.

    Action(0)
  }

  /// Returns a mutable reference to the specified child. Lazily creates the
  /// child if it is not present.
  fn mut_child(&mut self, action: Action) -> &mut ChanceNode {
    if !self.children.contains_key(&action) {
      let info = self.environment_info;
      self.children.insert(action, Box::new(ChanceNode::new(info)));
    }
    return &mut **self.children.get_mut(&action).unwrap();
  }
}

impl Node for ActionNode {
  fn mean_reward(&self) -> Reward { self.mean_reward }
  fn visits(&self) -> usize { self.visits }

  fn sample(&mut self, horizon: usize) -> Reward {
    if horizon == 0 {
      return Reward(0.0);  // Chose as the additive neutral element.
    }
    // TODO(dinowernli): Implement.
    return self.sample(horizon - 1);
  }
}

