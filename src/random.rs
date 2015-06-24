/// Basic random number generator. May be predictable.
pub trait Random {
  /// Returns a random number in the range [0, limit - 1].
  fn next_modulo(&mut self, limit: i64) -> i64;
}

/// Default implementation of the Random trait.
pub struct RandomImpl {
  state: i64,
}

impl RandomImpl {
  pub fn new(seed: i64) -> RandomImpl {
    RandomImpl {
      state: seed,
    }
  }

  /// Returns an arbitrary random number.
  pub fn next(&mut self) -> i64 {
    let result = self.state;
    self.state = (self.state * 7) % 589671;
    return result;
  }

  /// Returns a new random number generator seeded with the
  /// next random number produced by this generator.
  pub fn new_child(&mut self) -> RandomImpl {
    return RandomImpl::new(self.next());
  }
}

impl Random for RandomImpl {
  fn next_modulo(&mut self, limit: i64) -> i64 {
    return self.next() % limit;
  }
}
