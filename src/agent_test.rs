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

use agent::Agent;
use bitstring::Bitstring;
use predictor::context_tree::Predictor;
use random::Random;
use types::{Observation, Reward};

#[test]
fn age() {
  let mut fake_random = FakeRandom::new();
  let fake_predictor = Box::new(FakePredictor::new());
  let mut agent = Agent::new(
      10, &mut fake_random, fake_predictor);

  assert_eq!(0, agent.age());
  agent.update(Observation(3), Reward(4.0));
  assert_eq!(1, agent.age());
}


// Fake predictor.

struct FakePredictor;

impl FakePredictor {
  fn new() -> Self {
    FakePredictor
  }
}

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


// Fake random number generator.

struct FakeRandom {
  value: u64,
}

impl FakeRandom {
  fn new() -> Self {
    return FakeRandom {
      value: 4,
    };
  }
}

impl Random for FakeRandom {
  #[allow(unused_variables)]  // Ignores the limit.
  fn next_modulo(&mut self, limit: u64) -> u64 {
    return self.value;
  }
}
