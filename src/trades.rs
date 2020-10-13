#![allow(dead_code)]
use crate::rest::RestClient;
use serde_json::from_str;
use std::error::Error;
use crate::models::Trade;

pub struct Trades {
  client: RestClient,
}

impl Trades {
  pub fn new() -> Trades {
    Trades {
      client: RestClient::new(None, None),
    }
  }

  pub fn hist<S>(&self, symbol: S) -> Result<Vec<Trade>, Box<dyn Error>>
  where
    S: ToString,
  {
    let uri = format!("trades/t{}/hist", &symbol.to_string());
    let response = self.client.get(uri)?;
    let trades: Vec<Trade> = from_str(&response)?;
    Ok(trades)
  }
}

#[cfg(test)]
mod test {
  use crate::trades::Trades;
  use std::error::Error;

  #[test]
  fn trades_trading() -> Result<(), Box<dyn Error>> {
    Trades::new().hist("BTCUSD")?;
    Ok(())
  }
}
