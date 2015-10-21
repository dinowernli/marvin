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
use bitstring::Bit;

#[test]
fn from_u64() {
  let bits = Bitstring::create_from_u64(6);
  assert_eq!("110", bits.to_string());
}

#[test]
fn size() {
  let bits = Bitstring::create_from_u64(17);
  assert_eq!(5, bits.len());
}

#[test]
fn zero() {
  let bits = Bitstring::create_from_u64(0);
  assert_eq!("0", bits.to_string());
}

#[test]
fn empty() {
  let bits = Bitstring::create_empty();
  assert_eq!("", bits.to_string());
}

#[test]
fn append() {
  let mut bits = Bitstring::new();
  bits.push(Bit::Zero);
  bits.push(Bit::Zero);
  bits.push(Bit::One);
  bits.push(Bit::One);
  bits.push(Bit::Zero);
  assert_eq!("00110", bits.to_string());
}

#[test]
fn from_string() {
  let bits = Bitstring::create_from_string("01001");
  assert_eq!("01001", bits.to_string());
}
