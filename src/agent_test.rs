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

use agent::{Agent, EnvironmentInfo};
use bitstring::Bitstring;
use explorer::{Explorer, ExplorerFactory};
use predictor::Predictor;
use types::{Action, Observation, Reward, SingleReward};

#[test]
fn age() {
  let mut agent = Agent::new(
      default_info(),
      Box::new(FakePredictor),
      Box::new(FakeExplorerFactory)
  );

  assert_eq!(0, agent.age());
  agent.update(Observation(3), SingleReward(4));
  assert_eq!(1, agent.age());
}

#[test]
fn reward() {
  let mut agent = Agent::new(
      default_info(),
      Box::new(FakePredictor),
      Box::new(FakeExplorerFactory)
  );

  assert_eq!(Reward(0.0), agent.total_reward());
  agent.update(Observation(3), SingleReward(4));
  assert_eq!(Reward(4.0), agent.total_reward());
}

fn default_info() -> EnvironmentInfo {
  EnvironmentInfo::new(
      10 /* num_actions */,
      SingleReward(-3) /* min_reward */,
      SingleReward(7) /* max_reward */,
  )
}

// Fake predictor.

struct FakePredictor;

impl Predictor for FakePredictor {
  fn history_size(&self) -> usize { 17 }

  fn revert_to_history_size(&mut self, target_size: usize) {
    #![allow(unused_variables)]
    // Do nothing.
  }

  fn update(&mut self, bits: &Bitstring) {
    #![allow(unused_variables)]
    // Do nothing.
  }

  fn predict(&mut self, bits: &Bitstring) -> f64 {
    let neg_len = -(bits.len() as i64);
    return (neg_len as f64).exp2();
  }
}


// Fake explorer, including factory.

struct FakeExplorerFactory;

impl ExplorerFactory for FakeExplorerFactory {
  fn create_monte_carlo_explorer(
      &self, predictor: &mut Predictor) -> Box<Explorer> {
    #![allow(unused_variables)]
    Box::new(FakeExplorer)
  }

  fn create_random_explorer(&self) -> Box<Explorer> {
    Box::new(FakeExplorer)
  }
}

struct FakeExplorer;

impl Explorer for FakeExplorer {
  fn explore(&mut self, environment_info: EnvironmentInfo) -> Action {
    #![allow(unused_variables)]
    Action(0)
  }
}

