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

use explorer::Explorer;
use predictor::Predictor;
use types::Action;

pub struct MonteCarloExplorer<'a> {
  predictor: &'a mut Predictor,
}

impl <'a> MonteCarloExplorer<'a> {
  pub fn new(predictor: &mut Predictor) -> MonteCarloExplorer {
    MonteCarloExplorer {
      predictor: predictor,
    }
  }
}

impl <'a> Explorer for MonteCarloExplorer<'a> {
  fn explore(&mut self, num_actions: i16) -> Action {
    #![allow(unused_variables)]
    // TODO(dinowernli): Use self.predictor to find the best action.
    return Action(0);
  }
}
