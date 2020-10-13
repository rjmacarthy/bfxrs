#![allow(dead_code)]
use std::error::Error;
use std::io::Read;

static API_URL: &'static str = "https://api.bitfinex.com/v2/";

#[derive(Debug)]
pub struct RestClient {
  key: String,
  secret: String,
}

impl RestClient {
  pub fn new(key: Option<String>, secret: Option<String>) -> RestClient {
    RestClient {
      key: key.unwrap_or("".into()),
      secret: secret.unwrap_or("".into()),
    }
  }

  pub fn get<S>(&self, path: S) -> Result<String, Box<dyn Error>>
  where
    S: ToString,
  {
    let uri = format!("{}{}", API_URL, path.to_string());
    let mut response = reqwest::get(&uri).unwrap();
    let mut content = String::new();
    println!("{}", content);
    response
      .read_to_string(&mut content)
      .expect("Couldn't read to string");
    Ok(content)
  }

}
