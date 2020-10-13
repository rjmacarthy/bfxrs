#![allow(dead_code)]
use crate::rest::RestClient;
use serde_json::from_str;
use std::error::Error;
use crate::models::{TradeTickers, FundingTickers};

pub struct Tickers {
  client: RestClient
}

impl Tickers {
  pub fn new () -> Tickers {
    Tickers {
      client : RestClient::new(None, None)
    }
  }

  pub fn trading<S>(&self, symbols : Vec<S>) -> Result<Vec<TradeTickers>, Box<dyn Error>> where S : ToString {
    let symbols_joined = &symbols.iter()
      .map(|x| String::from("t") + &x.to_string())
      .collect::<Vec<_>>()
      .join(",");
      
    let uri = format!("tickers?symbols={}", symbols_joined);
    let response = self.client.get(uri)?;
    let tickers : Vec<TradeTickers> = from_str(&response)?;
    Ok(tickers)
  }

  pub fn funding<S>(&self, symbols : Vec<S>) -> Result<Vec<FundingTickers>, Box<dyn Error>> where S : ToString {
    let symbols_joined = &symbols
      .iter()
      .map(|x| String::from("f") + &x.to_string())
      .collect::<Vec<_>>().join(",");
    let uri = format!("tickers?symbols={}", symbols_joined);
    let response = self.client.get(uri)?;
    let tickers : Vec<FundingTickers> = from_str(&response)?;
    Ok(tickers)
  }
}

#[cfg(test)]
mod test {
  use crate::tickers::Tickers;
  use std::error::Error;

  #[test]
  fn tickers_trading() -> Result<(), Box<dyn Error>> {
    Tickers::new().trading(vec!["BTCUSD", "LTCUSD"])?;
    Ok(())
  }

  #[test]
  fn tickers_funding() -> Result<(), Box<dyn Error>> {
    Tickers::new().funding(vec!["USD", "BTC"])?;
    Ok(())
  }
}