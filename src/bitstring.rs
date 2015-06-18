#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bit {
  Zero,
  One,
}

pub struct Bitstring {
  // TODO(dinowernli): Investigate storing a Vec<u64> instead
  // and doing the conversoin to {Zero, One} upon request.
  bits: Vec<Bit>,
}

/// A type representing a sequence of Bits.
impl Bitstring {
  pub fn empty() -> Bitstring {
    return Bitstring {
      bits: Vec::new(),
    };
  }

  fn new(bits: Vec<Bit>) -> Bitstring {
    return Bitstring {
      bits: bits
    };
  }

  /// Encodes the supplied value as a Bitstring by taking its binary
  /// representation. The length of the resulting Bitstring is the
  /// smallest sequence of bits which can represent the value, i.e.,
  /// log2(ceil(value)).
  pub fn from_u64(value: u64) -> Bitstring {
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
    return Bitstring::new(bits);
  }

  pub fn bits(&self) -> &Vec<Bit> {
    return &self.bits;
  }

  pub fn len(&self) -> usize {
    return self.bits().len();
  }

  pub fn bit(&self, i: usize) -> Bit {
    return self.bits[i].clone();
  }
}
