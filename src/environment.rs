pub trait Environment {
  fn reward(&self) -> f64;
  fn observation(&self) -> i16;

  fn update(&mut self, action: i16);
}

pub struct CoinFlip {
  last_toss: bool,
}

impl CoinFlip {
  pub fn new() -> CoinFlip {
    CoinFlip {
      last_toss: false,  // tails
    }
  }
}

//impl Environment for CoinFlip {
//}
