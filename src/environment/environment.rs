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
use random::Random;
use types::{Action, Observation, Reward, SingleReward};

/// Models a general environment an agent can interact with and
/// learn from.
pub trait Environment {
  /// Returns basic information about the environment.
  fn info(&self) -> EnvironmentInfo;

  /// Returns the latest reward. Only call this after the first call to update.
  fn reward(&self) -> SingleReward;

  /// Returns the latest observation. Only call this after the first call to
  /// update.
  fn observation(&self) -> Observation;

  /// Recomputes the values of observation and reward based on a taken action.
  /// The supplied action must be within the range advertised in info() or else
  /// the behavior is unspecified.
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
  fn info(&self) -> EnvironmentInfo {
    EnvironmentInfo::new(
        2 /* num_actions */,
        SingleReward(0) /* min_reward */,
        SingleReward(1) /* max_reward */,
    )
  }

  fn reward(&self) -> SingleReward {
    // It is invalid to ask for a reward before the first update(), so we
    // unwrap the last guess.
    let is_correct = self.last_guess.unwrap() == self.last_toss;
    return SingleReward(match is_correct {
      true => 1,
      false => 0,
    });
  }

  fn observation(&self) -> Observation {
    match self.last_toss {
      CoinToss::Heads => Observation(0),
      CoinToss::Tails => Observation(1),
    }
  }

  fn update(&mut self, action: Action) {
    let num_actions = self.info().num_actions();

    self.last_guess = match action {
      Action(0) => Some(CoinToss::Heads),
      Action(1) => Some(CoinToss::Tails),
      Action(n) =>
          panic!("Got {:?} but num_actions is {}", action, num_actions),
    };

    self.last_toss = match self.random.next_modulo(2) {
      0 => CoinToss::Heads,
      1 => CoinToss::Tails,
      _ => panic!(),
    };
  }
}
