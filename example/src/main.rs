use bfxrs::bfx::{Bfx};
use bfxrs::time_frame::{TimeFrame};

fn main() {
  let bfx = Bfx::new();
  let tickers = bfx.tickers.trading(vec!["BTCUSD", "LTCUSD"]);
  let ticker = bfx.ticker.trading("BTCUSD");
  let candles = bfx.candles.hist(TimeFrame::T1D, "BTCUSD");
  println!("{:?}", tickers);
  println!("{:?}", ticker);
  println!("{:?}", candles);
}

