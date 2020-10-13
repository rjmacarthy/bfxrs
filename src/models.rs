use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
  mts: i64,
  value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candle {
  mts : i64,
  open : f64,
  close : f64,
  high: f64,
  low : f64,
  volume : f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookTrade {
  price: f64,
  count: i64,
  amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookFund {
  rate: f64,
  period: f64,
  count: i64,
  amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeTicker {
  pub bid: f64,
  pub bid_size: f64,
  pub ask: f64,
  pub ask_size: f64,
  pub daily_change: f64,
  pub daily_change_perc: f64,
  pub last_price: f64,
  pub volume: f64,
  pub high: f64,
  pub low: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingTicker {
  pub frr: f64,
  pub bid: f64,
  pub bid_period: i64,
  pub bid_size: f64,
  pub ask: f64,
  pub ask_period: i64,
  pub ask_size: f64,
  pub daily_change: f64,
  pub daily_change_perc: f64,
  pub last_price: f64,
  pub volume: f64,
  pub high: f64,
  pub low: f64,
  #[serde(skip_serializing)]
  _placeholder_1: Option<String>,
  #[serde(skip_serializing)]
  _placeholder_2: Option<String>,
  #[serde(skip_serializing)]
  _placeholder_3: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeTickers {
  pub symbol : String,
  pub bid: f64,
  pub bid_size : f64,
  pub ask : f64,
  pub ask_size : f64,
  pub daily_change : f64,
  pub daily_change_perc : f64,
  pub last_price : f64,
  pub volume : f64,
  pub high : f64,
  pub low : f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingTickers {
  pub symbol : String,
  pub ffr: f64,
  pub bid : f64,
  pub bid_size: i64,
  pub bid_period : f64,
  pub ask : f64,
  pub ask_size: f64,
  pub ask_period : f64,
  pub daily_change : f64,
  pub daily_change_perc : f64,
  pub last_price : f64,
  pub volume : f64,
  pub high : f64,
  pub low : f64,
  #[serde(skip_serializing)]
  _placeholder_1: Option<String>,
  #[serde(skip_serializing)]
  _placeholder_2: Option<String>,
  #[serde(skip_serializing)]
  _placeholder_3: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
  pub id: i64,
  pub mts: i64,
  pub amount: f64,
  pub price: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Funding {
  pub id: i64,
  pub mts: i64,
  pub amount: f64,
  pub price: f64,
  pub period: i64,
}