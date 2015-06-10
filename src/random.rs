pub struct Random {
  state: i64,
}

impl Random {
  pub fn new(seed: i64) -> Random {
    Random {
      state: seed,
    }
  }

  pub fn next(&mut self) -> i64 {
    let result = self.state;
    self.state = (self.state * 7) % 589671;
    return result;
  }

  /// Returns a random number in the range [0, limit - 1].
  pub fn next_modulo(&mut self, limit: i64) -> i64 {
    return self.next() % limit;
  }
}
