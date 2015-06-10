pub trait Environment {
  fn num_actions(&self) -> i16;
  fn reward(&self) -> f64;
  fn observation(&self) -> i16;

  fn update(&mut self, action: i16);
}

pub struct CoinFlip {
  last_toss: i16,
  last_guess: i16,
}

impl CoinFlip {
  pub fn new() -> CoinFlip {
    CoinFlip {
      last_toss: 0,   // tails

      // TODO(dinowernli): Add an enum for guesses for either
      // HEADS, TAILS. or INVALID.
      last_guess: -1,
    }
  }
}

impl Environment for CoinFlip {
  fn num_actions(&self) -> i16 {
    return 2;
  }

  fn reward(&self) -> f64 {
    return if self.last_guess == self.last_toss { 1.0 } else { 0.0 };
  }

  fn observation(&self) -> i16 {
    return self.last_toss;
  }

  fn update(&mut self, action: i16) {
    self.last_guess = match action {
      0 => 0,
      1 => 1,
      _ => -1,  // Invalid.
    };

    // Just alternate heads and tails for now.
    self.last_toss = match self.last_toss {
      0 => 1,
      1 => 0,
      _ => 0,  // Can't happen.
    };
  }
}
