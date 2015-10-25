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

use std::ops::{Add, Div, Sub};

/// A generic representation of cumulative rewards.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Reward(pub f64);

impl Add for Reward {
  type Output = Reward;

  fn add(self, rhs: Reward) -> Reward {
    let Reward(value) = self;
    let Reward(rhvalue) = rhs;
    return Reward(value + rhvalue);
  }
}

impl Add<SingleReward> for Reward {
  type Output = Reward;

  fn add(self, rhs: SingleReward) -> Reward {
    let Reward(value) = self;
    let SingleReward(rhvalue) = rhs;
    return Reward(value + rhvalue as f64);
  }
}

impl Div<f64> for Reward {
  type Output = Reward;

  fn div(self, rhs: f64) -> Reward {
    let Reward(value) = self;
    return Reward(value / rhs);
  }
}

/// The representation of a single reward as directly returned by the
/// environment.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SingleReward(pub i16);

impl Sub for SingleReward {
  type Output = SingleReward;

  fn sub(self, rhs: SingleReward) -> SingleReward {
    let SingleReward(value) = self;
    let SingleReward(rhvalue) = rhs;
    return SingleReward(value - rhvalue);
  }
}


#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Observation(pub i16);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Percept(pub Observation, pub SingleReward);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Action(pub i16);
