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

use std::string::String;
use std::string::ToString;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bit {
  Zero,
  One,
}

/// Basic representation of a sequence of bits.
pub struct Bitstring {
  // TODO(dinowernli): Investigate storing a Vec<u64> instead
  // and doing the conversoin to {Zero, One} upon request.
  bits: Vec<Bit>,
}

/// A type representing a sequence of Bits.
impl Bitstring {
  pub fn new() -> Self {
    return Bitstring {
      bits: Vec::new(),
    };
  }

  pub fn create_empty() -> Self {
    Bitstring::new()
  }

  /// Encodes the supplied value as a Bitstring by taking its binary
  /// representation. The length of the resulting Bitstring is the
  /// smallest sequence of bits which can represent the value, i.e.,
  /// log2(ceil(value)).
  pub fn create_from_u64(value: u64) -> Self {
    if value == 0 {
      return Bitstring::create_from_bits(vec!(Bit::Zero));
    }

    // We compute the bit values by repeatedly getting the least
    // significant bit and then shifting. Since pushing to the end of
    // a vector is more efficient than pushing to the start, we keep
    // appending more significant bits and then flip everything once
    // at the end.
    let mut bits = Vec::new();
    let mut remaining = value;
    while remaining > 0 {
      let least = remaining & 0x1;
      bits.push(if least == 1 { Bit::One } else { Bit::Zero });
      remaining = remaining >> 1;
    }

    bits.reverse();
    return Bitstring::create_from_bits(bits);
  }

  /// Returns a bitstring created from a sequence of characters, each of which
  /// must be either '0' or '1'. Panics if a bad string is passed.
  pub fn create_from_string(value: &str) -> Self {
    let mut bits = Vec::new();
    for b in value.chars() {
      bits.push(match b {
        '0' => Bit::Zero,
        '1' => Bit::One,
        _ => panic!("Could not extract bitstring from {}", value),
      });
    }
    return Bitstring::create_from_bits(bits);
  }

  fn create_from_bits(bits: Vec<Bit>) -> Self {
    return Bitstring {
      bits: bits
    };
  }

  pub fn bits(&self) -> &Vec<Bit> { &self.bits }
  pub fn len(&self) -> usize { self.bits().len() }
  pub fn bit(&self, i: usize) -> Bit { self.bits[i] }

  pub fn push(&mut self, bit: Bit) {
    self.bits.push(bit);
  }

  /// Only valid if this bit string is not empty.
  pub fn pop(&mut self) -> Bit {
    return self.bits.pop().unwrap();
  }
}

impl ToString for Bitstring {
  /// Returns the string representation of the bitstring, i.e., the
  /// string representation of 5 is '101'.
  fn to_string(&self) -> String {
    let mut result = String::new();
    for bit in self.bits() {
      result.push(match *bit {
        Bit::Zero => '0',
        Bit::One => '1',
      });
    }
    return result;
  }
}
