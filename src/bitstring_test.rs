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
