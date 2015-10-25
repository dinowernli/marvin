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
use types::{Action, Percept, Reward, SingleReward};

use std::collections::{HashMap, BTreeSet};

/// The number of visits we require to child action nodes before starting to
/// trade off explore and exploit.
const MIN_VISITS: usize = 1;

/// Corresponds to the "C" in the UCB description. Increasing this value result
/// in more weight being put on explore vs. exploit. Should always be positive.
const EXPLORE_RATIO: f64 = 0.2;

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

    // TODO(dinowernli): The calls below prevent a few unused-warnings. Remove
    // them once this method is fully implemented.
    let action = tree.select_explore_exploit(&mut *self.random, 14);
    self.predictor.history_size();

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
  fn select_explore_exploit(
      &mut self,
      random: &mut Random,
      remaining_horizon: usize) -> Action {
    // Stores the children, if any, which *need* to be visited.
    let mut require_visit: BTreeSet<Action> = BTreeSet::new();

    // Stores the children,if any with the highest UCB score.
    let mut max_ucb_children: BTreeSet<Action> = BTreeSet::new();
    let mut max_ucb_score: Option<f64> = None;

    // Go through all actions and check the status of the corresponding child.
    for a in 0..self.environment_info.num_actions() - 1 {
      let action = Action(a);

      if self.mut_child(action).requires_visit() {
        require_visit.insert(action);
        continue;
      }

      // If we have nodes requiring a visit, we can save ourselves the trouble.
      if require_visit.is_empty() {
        let ucb = self.ucb_score(action, remaining_horizon);

        if max_ucb_score.is_none() || ucb > max_ucb_score.unwrap() {
          max_ucb_children.clear();
          max_ucb_score = Some(ucb);
        }

        if max_ucb_score.is_some() && ucb == max_ucb_score.unwrap() {
          max_ucb_children.insert(action);  // Also if UCB is the new max.
        }
      }
    }

    // Prioritize children requiring visits, and take the max UCB otherwise.
    if !require_visit.is_empty() {
      return self.pick_random(&require_visit, random);
    } else {
      debug_assert!(!max_ucb_children.is_empty());
      return self.pick_random(&max_ucb_children, random);
    }
  }

  /// Returns an action from the set, uniformly at random.
  fn pick_random(&self, set: &BTreeSet<Action>, random: &mut Random) -> Action {
    let index = random.next_modulo(set.len() as u64);
    return *set.iter().nth(index as usize).unwrap();
  }

  /// Computes the score of this action as given by the UCB heuristic. Assumes
  /// that the child node for this action exists and has been visited at least
  /// once.
  fn ucb_score(&mut self, action: Action, remaining_horizon: usize) -> f64 {
    // Grab a few values before we borrow self below.
    let visits = self.visits() as f64;
    let SingleReward(reward_range) = self.environment_info.reward_range();
    let child: &ChanceNode = self.mut_child(action);

    let reward_scale = reward_range as f64 * remaining_horizon as f64;
    let Reward(exploitation) = child.mean_reward() / reward_scale;
    debug_assert!(exploitation >= 0.0);
    debug_assert!(exploitation <= 1.0);

    let visit_ratio = visits.log2() / child.visits() as f64;
    let exploration = EXPLORE_RATIO * visit_ratio.sqrt();

    let ucb = exploration + exploitation;
    debug_assert!(ucb >= 0.0);

    return ucb;
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

