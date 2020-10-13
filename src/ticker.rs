#![allow(dead_code)]
use crate::rest::RestClient;
use serde_json::from_str;
use std::error::Error;
use crate::models::{TradeTicker, FundingTicker};

pub struct Ticker {
  client: RestClient,
}
impl Ticker {
  pub fn new() -> Ticker {
    Ticker {
      client: RestClient::new(None, None),
    }
  }

  pub fn trading<S>(&self, symbol: S) -> Result<TradeTicker, Box<dyn Error>>
  where
    S: ToString,
  {
    let uri = format!("ticker/t{}", &symbol.to_string());
    let response = self.client.get(uri)?;
    let ticker: TradeTicker = from_str(&response)?;
    Ok(ticker)
  }

  pub fn funding<S>(&self, symbol: S) -> Result<FundingTicker, Box<dyn Error>>
  where
    S: ToString,
  {
    let uri = format!("ticker/f{}", &symbol.to_string());
    let response = self.client.get(uri)?;
    let ticker: FundingTicker = from_str(&response)?;
    Ok(ticker)
  }
}

#[cfg(test)]
mod test {
  use crate::ticker::Ticker;
  use std::error::Error;

  #[test]
  fn ticker_trading() -> Result<(), Box<dyn Error>> {
    Ticker::new().trading("BTCUSD")?;
    Ticker::new().trading("LTCBTC")?;
    Ok(())
  }

  #[test]
  fn ticker_funding() -> Result<(), Box<dyn Error>> {
    Ticker::new().funding("USD")?;
    Ticker::new().funding("BTC")?;
    Ok(())
  }
}
