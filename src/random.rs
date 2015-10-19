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

use rand::{Rng, SeedableRng, StdRng};

/// Basic random number generator (not necessarily cryptographically secure).
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
      generator: SeedableRng::from_seed(seed_slice),
    };
  }

  /// Returns a new random number generator seeded with the
  /// next random number produced by this generator.
  pub fn new_child(&mut self) -> Self {
    return RandomImpl::create(self.next() as usize);
  }

  /// Returns a random number.
  fn next(&mut self) -> u64 {
    self.generator.gen::<u64>()
  }
}

impl Random for RandomImpl {
  fn next_modulo(&mut self, limit: u64) -> u64 {
    return self.next() % limit;
  }
}
