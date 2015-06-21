use std::ops::{Add, Div};

#[derive(Copy, Clone, Debug)]
pub struct Reward(pub f64);

impl Add for Reward {
  type Output = Reward;

  fn add(self, rhs: Reward) -> Reward {
    let Reward(value) = self;
    let Reward(rhvalue) = rhs;
    return Reward(value + rhvalue);
  }
}

impl Div<f64> for Reward {
  type Output = Reward;

  fn div(self, rhs: f64) -> Reward {
    let Reward(value) = self;
    return Reward(value / rhs);
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Observation(pub i16);

#[derive(Copy, Clone, Debug)]
pub struct Action(pub i16);
