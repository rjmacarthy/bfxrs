#![allow(dead_code)]
use crate::rest::RestClient;
use serde_json::from_str;
use std::error::Error;

struct Platform {
  client: RestClient
}

impl Platform {
  pub fn new() -> Platform {
    Platform {
      client: RestClient::new(None, None),
    }
  }

  pub fn status(&self) -> Result<Vec<i8>, Box<dyn Error>> {
    let uri = String::from("platform/status");
    let response = self.client.get(uri)?;
    let status: Vec<i8> = from_str(&response)?;
    Ok(status)
  }
}

#[cfg(test)]
mod test {
  use crate::platform::Platform;
  use std::error::Error;

  #[test]
  fn platform_status() -> Result<(), Box<dyn Error>> {
    Platform::new()
      .status()
      .expect("Failed to get platform status");
    Ok(())
  }
}
