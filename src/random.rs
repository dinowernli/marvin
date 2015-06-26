use rand::{Rng, SeedableRng, StdRng};

/// Basic random number generator. May be predictable.
pub trait Random {
  /// Returns a random number in the range [0, limit - 1].
  fn next_modulo(&mut self, limit: u64) -> u64;
}

/// Default implementation of the Random trait.
pub struct RandomImpl {
  generator: StdRng,
}

impl RandomImpl {
  pub fn create(seed: usize) -> Self {
    let seed_slice: &[_] = &[seed as usize];
    return RandomImpl {
      //generator: SeedableRng::from_seed(seed_slice),
      generator: SeedableRng::from_seed(seed_slice),
    };
  }

  /// Returns an arbitrary random number.
  pub fn next(&mut self) -> u64 {
    self.generator.gen::<u64>()
  }

  /// Returns a new random number generator seeded with the
  /// next random number produced by this generator.
  pub fn new_child(&mut self) -> Self {
    return RandomImpl::create(self.next() as usize);
  }
}

impl Random for RandomImpl {
  fn next_modulo(&mut self, limit: u64) -> u64 {
    return self.next() % limit;
  }
}
