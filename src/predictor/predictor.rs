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

use bitstring::Bitstring;

/// An object capable of predicting observations and rewards based on
/// experience. Predictors have an abstract notion of history which
/// grows over time and represents the experience.
pub trait Predictor {
  /// Returns the size of the currently tracked history.
  fn history_size(&self) -> usize;

  /// Appends the provided bit string to the tracked history.
  fn update(&mut self, bitstring: &Bitstring);

  /// Reverts the context tree to a previous state by undoing update
  /// operations. The specified size must be at most the current size.
  fn revert_to_history_size(&mut self, target_size: usize);

  /// Returns the probability, given the current history, that "bits" are the
  /// next observed symbols.
  fn predict(&mut self, bits: &Bitstring) -> f64;
}

