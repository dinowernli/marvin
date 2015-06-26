use random::Random;

/// Models a general environment an agent can interact with and
/// learn from. Know how many actions are availavle and knows
/// how to process the actions of agent.
pub trait Environment {
  fn num_actions(&self) -> i16;
  fn reward(&self) -> f64;
  fn observation(&self) -> i16;

  fn update(&mut self, action: i16);
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
  pub fn new(random: &'a mut Random) -> Self<'a> {
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

  fn reward(&self) -> f64 {
    match self.last_guess  {
      Some(val) => if val == self.last_toss { 11.0 } else { 10.0 },
      _ => 0.0,
    }
  }

  fn observation(&self) -> i16 {
    match self.last_toss  {
      CoinToss::Heads => 0,
      CoinToss::Tails => 1,
    }
  }

  fn update(&mut self, action: i16) {
    self.last_guess = match action {
      0 => Some(CoinToss::Heads),
      1 => Some(CoinToss::Tails),
      _ => None,
    };

    // Just alternate heads and tails for now.
    let r = self.random.next_modulo(2);
    self.last_toss = if r == 1 { CoinToss::Heads } else { CoinToss::Tails};
  }
}
