pub trait Environment {
  fn reward(&self) -> f64;
  fn observation(&self) -> i16;

  fn update(&mut self, action: i16);
}

pub struct CoinFlip {
  last_toss: bool,
  last_guess: bool,
}

impl CoinFlip {
  pub fn new() -> CoinFlip {
    CoinFlip {
      last_toss: false,   // tails

      // TODO make this optional.
      last_guess: false,  // tails
    }
  }
}

impl Environment for CoinFlip {
  fn reward(&self) -> f64 {
    return if self.last_guess == self.last_toss { 1.0 } else { 0.0 };
  }

  fn observation(&self) -> i16 {
    return self.last_toss as i16;
  }

  fn update(&mut self, action: i16) {
    self.last_toss = !self.last_toss;  // just flip over for now
  }
}
