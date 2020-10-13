use std::fmt;

#[derive(Debug)]
pub enum TimeFrame {
  T1m,
  T5m,
  T15m,
  T30m,
  T1h,
  T3h,
  T6h,
  T12h,
  T1D,
  T7D,
  T14D,
  T1M
}

impl fmt::Display for TimeFrame {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}