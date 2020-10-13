use std::fmt;

#[derive(Debug)]
pub enum Precision {
  P0,
  P1,
  P2,
  P3,
  P4,
  R0,
}

impl fmt::Display for Precision {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[cfg(test)]
mod test {
  use crate::precision::Precision;

  #[test]
  fn precision() {
    assert_eq!(Precision::P0.to_string(), "P0");
    assert_eq!(Precision::P1.to_string(), "P1");
    assert_eq!(Precision::P2.to_string(), "P2");
    assert_eq!(Precision::P3.to_string(), "P3");
    assert_eq!(Precision::P4.to_string(), "P4");
    assert_eq!(Precision::R0.to_string(), "R0");
  }

}
