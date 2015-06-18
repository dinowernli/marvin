use bitstring::Bitstring;
use bitstring::Bit;

#[test]
fn from_u64() {
  let bits = Bitstring::from_u64(6);  // 110.
  assert_eq!(3, bits.len());
  assert_eq!(Bit::One, bits.bit(0));
  assert_eq!(Bit::One, bits.bit(1));
  assert_eq!(Bit::Zero, bits.bit(2));
}

#[test]
fn empty() {
  let bits = Bitstring::from_u64(0);
  assert_eq!(1, bits.len());
  assert_eq!(Bit::Zero, bits.bit(0));
}
