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
fn empty_probability() {
  let mut tree = ContextTree::create(0);
  let block_prob = tree.log_block_prob().exp2();
  assert_almost_eq(1.0, block_prob, EPS);
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
  assert!(diff < tol, "diff = {}, but expected less than {}", diff, tol);
  assert!(diff > -tol, "diff = {}, but expected at least {}", diff, tol);
}
