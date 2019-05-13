use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
  api_url: String,
  app_id: String,
  api_key: String,
  lang: String,
}

impl Config {
  pub fn new() -> Self {
    dotenv().ok();
    Config {
      api_url: Config::getVar(
        "API_URL",
        "https://od-api.oxforddictionaries.com:443/api/v2/entries",
      ),
      app_id: Config::getVar("APP_ID", ""),
      api_key: Config::getVar("API_KEY", ""),
      lang: Config::getVar("LANG", "en"),
    }
  }

  fn getVar(key: &str, default: &str) -> String {
    if let Ok(val) = env::var(key) {
      val
    } else {
      String::from(default)
    }
  }
}
