#![allow(dead_code)]
use crate::rest::RestClient;
use serde_json::from_str;
use std::error::Error;
use crate::models::Stat;

pub struct Stats {
  client: RestClient,
}
impl Stats {
  pub fn new() -> Stats {
    Stats {
      client: RestClient::new(None, None),
    }
  }

  pub fn trading<S>(&self, key: S, size: S, symbol: S, side: S) -> Result<Stat, Box<dyn Error>>
  where
    S: ToString,
  {
    let uri = format!(
      "stats1/{}:{}:{}:{}/last",
      key.to_string(),
      size.to_string(),
      symbol.to_string(),
      side.to_string()
    );

    let response = self.client.get(uri)?;

    let stat: Stat = from_str(&response)?;

    Ok(stat)
  }

  pub fn history<S>(&self, key: S, size: S, symbol: S, side: S) -> Result<Vec<Stat>, Box<dyn Error>>
  where
    S: ToString,
  {
    let uri = format!(
      "stats1/{}:{}:{}:{}/hist",
      key.to_string(),
      size.to_string(),
      symbol.to_string(),
      side.to_string()
    );

    let response = self.client.get(uri)?;

    let stat: Vec<Stat> = from_str(&response)?;

    Ok(stat)
  }
}

#[cfg(test)]
mod test {
  use crate::stats::Stats;
  use std::error::Error;
  #[test]
  fn stats_trading() -> Result<(), Box<dyn Error>> {
    Stats::new().trading("pos.size", "1m", "tBTCUSD", "long")?;
    Ok(())
  }

  #[test]
  fn stats_history() -> Result<(), Box<dyn Error>> {
    Stats::new().trading("pos.size", "1m", "tBTCUSD", "short")?;
    Ok(())
  }
}
