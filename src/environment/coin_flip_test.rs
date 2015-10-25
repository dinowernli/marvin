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
use environment::coin_flip::CoinFlip;
use environment::environment::Environment;
use random::Random;
use types::{Action, Observation, SingleReward};

#[test]
fn info() {
  let mut random = FakeRandom;
  let mut coin_flip = CoinFlip::new(&mut random);
  let info = coin_flip.info();
  assert_eq!(2, info.num_actions());
  assert_eq!(SingleReward(0), info.min_reward());
  assert_eq!(SingleReward(1), info.max_reward());
}

#[test]
fn correct_guess() {
  let mut random = FakeRandom;
  let mut coin_flip = CoinFlip::new(&mut random);

  coin_flip.update(Action(0));  // Correct guess.
  assert_eq!(SingleReward(1), coin_flip.reward());
  assert_eq!(Observation(0), coin_flip.observation());
}

#[test]
#[should_panic]
fn panics_if_no_update() {
  let mut random = FakeRandom;
  let mut coin_flip = CoinFlip::new(&mut random);
  coin_flip.reward();
}

struct FakeRandom;

impl Random for FakeRandom {
  fn next_modulo(&mut self, limit: u64) -> u64 {
    if limit != 2 {
      panic!("Coin flip should only use modulo 2, but got {}", limit);
    }
    return 0;  // Heads.
  }
}

