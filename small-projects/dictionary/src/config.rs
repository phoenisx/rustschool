use dotenv::dotenv;

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
      api_url: Config::get_var(
        "API_URL",
        "https://od-api.oxforddictionaries.com:443/api/v2/entries",
      ),
      app_id: Config::get_var("APP_ID", ""),
      api_key: Config::get_var("API_KEY", ""),
      lang: Config::get_var("LANG", "en"),
    }
  }

  fn get_var(key: &str, default: &str) -> String {
    if let Ok(val) = dotenv::var(key) {
      val
    } else {
      String::from(default)
    }
  }
}
