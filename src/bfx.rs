#![allow(dead_code)]
use crate::books::Books;
use crate::stats::Stats;
use crate::ticker::Ticker;
use crate::tickers::Tickers;
use crate::trades::Trades;
use crate::candles::Candles;

pub struct Bfx {
  pub books: Books,
  pub stats : Stats,
  pub ticker: Ticker,
  pub tickers : Tickers,
  pub trades : Trades,
  pub candles : Candles
}
impl Bfx {
  pub fn new () -> Bfx {
    Bfx {
     books: Books::new(),
     stats: Stats::new(),
     ticker: Ticker::new(),
     tickers :Tickers::new(),
     trades :Trades::new(),
     candles :Candles::new(),
    }
  }
}