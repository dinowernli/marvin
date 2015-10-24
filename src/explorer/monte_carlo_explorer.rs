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
use types::{Action, Percept, Reward};

use std::collections::HashMap;

pub struct MonteCarloExplorer<'a> {
  predictor: &'a mut Predictor,
}

impl <'a> MonteCarloExplorer<'a> {
  pub fn new(predictor: &mut Predictor) -> MonteCarloExplorer {
    MonteCarloExplorer {
      predictor: predictor,
    }
  }
}

impl <'a> Explorer for MonteCarloExplorer<'a> {
  fn explore(&mut self, num_actions: i16) -> Action {
    #![allow(unused_variables)]
    // TODO(dinowernli): Use self.predictor to find the best action.
    return Action(0);
  }
}

trait Node {
  fn mean_reward(&self) -> Reward;
}

struct ActionNode <'a> {
  visits: u64,
  mean_reward: Reward,
  children: Box<HashMap<Percept, Box<Node + 'a>>>,
}

impl <'a> ActionNode<'a> {
  fn mut_child(&'a mut self, percept: Percept) -> &'a mut Node {
    if !self.children.contains_key(&percept) {
      self.children.insert(percept, Box::new(ChanceNode::new()));
    }
    return &mut **self.children.get_mut(&percept).unwrap();
  }
}

impl <'a> Node for ActionNode<'a> {
  fn mean_reward(&self) -> Reward { self.mean_reward }
}

struct ChanceNode <'a> {
  visits: u64,
  mean_reward: Reward,
  children: HashMap<Action, Box<Node + 'a>>,
}

impl <'a> ChanceNode<'a> {
  fn new() -> ChanceNode<'a> {
    ChanceNode {
      visits: 0,
      mean_reward: Reward(0.0),
      children: HashMap::new(),
    }
  }
}

impl <'a> Node for ChanceNode<'a> {
  fn mean_reward(&self) -> Reward { self.mean_reward }
}

