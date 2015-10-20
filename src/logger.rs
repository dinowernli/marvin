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

/// A logging framework for this application. Right now it just logs to stdout,
/// but it should eventually support logging AI progress to a file.

extern crate log;

use log::{LogLevel, LogMetadata, LogRecord};

/// A logger which just logs everything to stdout.
pub struct StdoutLogger {
  /// The highest log level for which we want log calls to take effect.
  max_enabled_level: LogLevel,
}

impl StdoutLogger {
  pub fn new(max_enabled_level: LogLevel) -> StdoutLogger {
    StdoutLogger {
      max_enabled_level: max_enabled_level
    }
  }
}

impl log::Log for StdoutLogger {
  fn enabled(&self, metadata: &LogMetadata) -> bool {
    metadata.level() <= self.max_enabled_level
  }

  fn log(&self, record: &LogRecord) {
    println!("{}:{} - {}", record.level(), record.target(), record.args());
  }
}
