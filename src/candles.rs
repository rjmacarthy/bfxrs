#![allow(dead_code)]
use crate::models::Candle;
use crate::rest::RestClient;
use crate::time_frame::TimeFrame;
use serde_json::from_str;
use std::error::Error;

pub struct Candles {
  client: RestClient,
}
impl Candles {
  pub fn new() -> Candles {
    Candles {
      client: RestClient::new(None, None),
    }
  }

  pub fn last<S>(&self, time_frame: TimeFrame, symbol: S) -> Result<Candle, Box<dyn Error>>
  where
    S: ToString,
  {
    let url = format!(
      "candles/trade:{}:t{}/last",
      time_frame.to_string().replace("T", ""),
      symbol.to_string()
    );
    let response = self.client.get(url)?;
    let candle: Candle = from_str(&response)?;
    Ok(candle)
  }

  pub fn hist<S>(&self, time_frame: TimeFrame, symbol: S) -> Result<Vec<Candle>, Box<dyn Error>>
  where
    S: ToString,
  {
    let uri = format!(
      "candles/trade:{}:t{}/hist",
      time_frame.to_string().replace("T", ""),
      symbol.to_string()
    );
    let response = self.client.get(uri)?;
    let candle: Vec<Candle> = from_str(&response)?;
    Ok(candle)
  }
}

#[cfg(test)]
mod test {
  use crate::candles::Candles;
  use crate::time_frame::TimeFrame;
  use std::error::Error;
  #[test]
  fn candle_last() -> Result<(), Box<dyn Error>> {
    Candles::new().last(TimeFrame::T1D, "BTCUSD")?;
    Candles::new().last(TimeFrame::T1m, "BTCUSD")?;
    Ok(())
  }

  #[test]
  fn candle_hist() -> Result<(), Box<dyn Error>> {
    Candles::new().hist(TimeFrame::T1D, "BTCUSD")?;
    Candles::new().hist(TimeFrame::T1m, "BTCUSD")?;
    Ok(())
  }

}
