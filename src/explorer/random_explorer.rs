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

use agent::EnvironmentInfo;
use explorer::Explorer;
use random::Random;
use types::Action;

pub struct RandomExplorer {
  random: Box<Random>
}

impl RandomExplorer {
  pub fn new(random: Box<Random>) -> RandomExplorer {
    RandomExplorer {
      random: random,
    }
  }
}

impl Explorer for RandomExplorer {
  fn explore(&mut self, environment_info: EnvironmentInfo) -> Action {
    let num_actions = environment_info.num_actions();
    Action(self.random.next_modulo(num_actions as u64) as i16)
  }
}
