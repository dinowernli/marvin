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
use explorer::monte_carlo_explorer::MonteCarloExplorer;
use explorer::random_explorer::RandomExplorer;
use predictor::Predictor;
use random::RandomImpl;

/// An object which knows how to produce explorers.
pub trait ExplorerFactory {
  fn create_monte_carlo_explorer<'a>(
      &self, predictor: &'a mut Predictor) -> Box<Explorer + 'a>;
  fn create_random_explorer(
      &self) -> Box<Explorer>;
}

pub struct ExplorerFactoryImpl;

impl ExplorerFactoryImpl {
  pub fn new() -> ExplorerFactoryImpl { ExplorerFactoryImpl }
}

impl ExplorerFactory for ExplorerFactoryImpl {
  fn create_monte_carlo_explorer<'a>(
      &self, predictor: &'a mut Predictor) -> Box<Explorer + 'a> {
    Box::new(MonteCarloExplorer::new(predictor))
  }

  fn create_random_explorer(&self) -> Box<Explorer> {
    Box::new(RandomExplorer::new(Box::new(RandomImpl::create(235669))))
  }
}
