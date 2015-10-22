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
use context_tree::ContextTree;
use context_tree::Predictor;

const EPS: f64 = 0.0000001;

#[test]
fn size() {
  let tree = ContextTree::create(3);
  assert_eq!(15, tree.size());  // 2^(n+1) - 1.
}

#[test]
fn empty_size() {
  let tree = ContextTree::create(0);
  assert_eq!(1, tree.size());
}

#[test]
fn predict_empty() {
  let mut tree = ContextTree::create(0);
  assert_almost_eq(1.0, tree.predict(&Bitstring::create_empty()), EPS);
}

#[test]
fn predict_uniform() {
  let mut tree = ContextTree::create(3);
  let prob = tree.predict(&Bitstring::create_from_string("01"));

  // Bitstring of length 2, not enough history, so uniform distribution.
  assert_almost_eq(0.25, prob, EPS);
}

#[test]
fn predict_with_history() {
  let mut tree = ContextTree::create(3);
  let bits = Bitstring::create_from_string("100");
  let history = Bitstring::create_from_string("10011110");

  // At first, uniform distribution for length 3.
  assert_almost_eq(0.125, tree.predict(&bits), EPS);

  tree.update(&history);

  // Now, expect the value we got from a reference implementation.
  assert_almost_eq(0.050951086956, tree.predict(&bits), EPS);
}

#[test]
fn revert_restores_size() {
  let mut tree = ContextTree::create(7);
  tree.update(&Bitstring::create_from_string("0100101001010000"));
  tree.revert_to_history_size(5);
  assert_eq!(5, tree.history_size());
}

// #[test]
// TODO(dinowernli): Enable once reverting is finalized.
fn revert_restores_state() {
  // Setup a tree with some arbitrary history which is long enough.
  let mut tree = ContextTree::create(7);
  tree.update(&Bitstring::create_from_string("01001010001001111"));
  let initial_size = tree.history_size();

  let bits = Bitstring::create_from_string("0010");
  let initial_prob = tree.predict(&bits);

  // Update the tree with some more history, then revert.
  tree.update(&Bitstring::create_from_string("01010010100"));
  tree.revert_to_history_size(initial_size);

  let final_size = tree.history_size();
  let final_prob = tree.predict(&bits);

  // The probability and size after reverting should be the same as before.
  assert_eq!(final_size, initial_size);
  assert_almost_eq(final_prob, initial_prob, EPS);
}

#[test]
fn noop_revert_is_valid() {
  let mut tree = ContextTree::create(3);
  tree.update(&Bitstring::create_from_string("00010"));
  let history_size = tree.history_size();
  tree.revert_to_history_size(history_size);  // Shouldn't crash.
}

#[test]
#[should_panic]
fn invalid_revert() {
  let mut tree = ContextTree::create(0);
  assert_eq!(0, tree.history_size());

  // Attempt to revert to something greater than the history size.
  tree.revert_to_history_size(17);
}

fn assert_almost_eq(expected: f64, actual: f64, tol: f64) {
  let diff = expected - actual;
  let message = format!(
      "expected {} but got {}, diff = {}", expected, actual, diff);
  assert!(-tol < diff && diff < tol, message);
}
