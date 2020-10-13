#![allow(dead_code)]
use crate::rest::RestClient;
use crate::precision::Precision;
use serde_json::from_str;
use std::error::Error;
use crate::models::{BookFund, BookTrade};

#[derive(Debug)]
pub struct Books {
  client: RestClient,
}

impl Books {
  pub fn new() -> Books {
    Books {
      client: RestClient::new(None, None),
    }
  }

  pub fn trading<S>(&self, symbol: S, precision: Precision) -> Result<Vec<BookTrade>, Box<dyn Error>>
  where
    S: ToString,
  {
    let uri = format!("book/t{}/{}", symbol.to_string(), precision.to_string());
    let response = self.client.get(uri)?;
    let book: Vec<BookTrade> = from_str(&response)?;
    Ok(book)
  }

  pub fn funding<S>(&self, symbol: S, precision: Precision) -> Result<Vec<BookFund>, Box<dyn Error>>
  where
    S: ToString,
  {
    let uri = format!("book/f{}/{}", symbol.to_string(), precision.to_string());
    let response = self.client.get(uri)?;
    let book: Vec<BookFund> = from_str(&response)?;
    Ok(book)
  }
}

#[cfg(test)]
mod test {
  use crate::books::Books;
  use crate::precision::Precision;
  use std::error::Error;

  #[test]
  pub fn books_trading() -> Result<(), Box<dyn Error>> {
    Books::new()
      .trading("BTCUSD", Precision::P0)
      .expect("Failed to get and parse book");
    Ok(())
  }

  #[test]
  fn books_funding() -> Result<(), Box<dyn Error>> {
    Books::new()
      .funding("BTC", Precision::P0)
      .expect("Failed to get and parse funding book");
    Ok(())
  }
}
