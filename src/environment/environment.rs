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

use random::Random;
use types::{Action, Observation, Reward};

/// Models a general environment an agent can interact with and
/// learn from. Know how many actions are availavle and knows
/// how to process the actions of agent.
pub trait Environment {
  fn num_actions(&self) -> i16;
  fn reward(&self) -> Reward;
  fn observation(&self) -> Observation;

  fn update(&mut self, action: Action);
}

/// An environment in which the observations represent repeated
/// tosses of a coin. Actions which guess the outcome of the coin
/// toss are rewarded.
pub struct CoinFlip<'a> {
  last_toss: CoinToss,
  last_guess: Option<CoinToss>,
  random: &'a mut Random,
}

/// The possible outcomes of a single coin toss.
#[derive(Copy, Clone, Debug, PartialEq)]
enum CoinToss {
  Heads,
  Tails,
}

impl <'a> CoinFlip<'a> {
  pub fn new(random: &'a mut Random) -> CoinFlip<'a> {
    CoinFlip {
      last_toss: CoinToss::Tails,
      last_guess: None,
      random: random,
    }
  }
}

impl <'a> Environment for CoinFlip<'a> {
  fn num_actions(&self) -> i16 {
    return 2;
  }

  fn reward(&self) -> Reward {
    Reward(match self.last_guess {
      Some(val) => if val == self.last_toss { 11.0 } else { 10.0 },
      _ => 0.0,
    })
  }

  fn observation(&self) -> Observation {
    match self.last_toss {
      CoinToss::Heads => Observation(0),
      CoinToss::Tails => Observation(1),
    }
  }

  fn update(&mut self, action: Action) {
    let Action(a) = action;
    debug_assert!(a >= 0 && a < self.num_actions());

    self.last_guess = match action {
      Action(0) => Some(CoinToss::Heads),
      Action(1) => Some(CoinToss::Tails),
      _ => None,
    };

    self.last_toss = match self.random.next_modulo(2) {
      0 => CoinToss::Heads,
      1 => CoinToss::Tails,
      _ => panic!(),
    };
  }
}
