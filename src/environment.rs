use random::Random;

pub trait Environment {
  fn num_actions(&self) -> i16;
  fn reward(&self) -> f64;
  fn observation(&self) -> i16;

  fn update(&mut self, action: i16);
}

pub struct CoinFlip {
  last_toss: CoinToss,
  last_guess: Option<CoinToss>,
  random: Box<Random>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum CoinToss {
  Heads,
  Tails,
}

impl CoinFlip {
  pub fn new(random: Box<Random>) -> CoinFlip {
    CoinFlip {
      last_toss: CoinToss::Tails,
      last_guess: None,
      random: random,
    }
  }
}

impl Environment for CoinFlip {
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
