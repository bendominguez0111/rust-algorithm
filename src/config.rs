use std::env;
use dotenv::dotenv;

#[allow(dead_code)]
pub enum Environment {
    Production,
    Paper
}

//static lifetime
pub fn universe() -> Vec<&'static str> {

    let universe = vec![
        "SPY", // S&P 500, Equity Index
        "META", // Facebook, Tech
        "F", // Ford, Auto manufacturers
        "AAPL", // Apple, Consumer Goods
        "INTC", // Intel Corp., Semiconductor
        "XOM", // Exxon Mobil, Energy
        "JPM", // JP Morgan, Banking
        "HD", // Home Depot, Consumer Durables
        "V", // Visa, Financials
        "AAL" // American Airlines, Airlines
    ];

    universe
}


pub struct Config {
    pub alpaca_base_url: String,
    pub alpaca_base_url_data: String,
    pub alpaca_api_key: String,
    pub alpaca_api_secret: String
}

impl Config {
    pub fn load(env: Environment) -> Self {
        
        //load dotenv vars
        dotenv().ok();

        let (alpaca_base_url, alpaca_base_url_data, alpaca_api_key, alpaca_api_secret) = match env {
            Environment::Production => (
                "ALPACA_BASE_URL_PROD",
                "ALPACA_BASE_URL_DATA",
                "ALPACA_API_KEY_PROD",
                "ALPACA_API_SECRET_PROD"
            ),
            Environment::Paper => (
                "ALPACA_BASE_URL_PAPER",
                "ALPACA_BASE_URL_DATA",
                "ALPACA_API_KEY_PAPER",
                "ALPACA_API_SECRET_PAPER",
            )
        };

        Config {
            alpaca_base_url: env::var(alpaca_base_url).expect(&format!("Could not find: {}", alpaca_base_url)),
            alpaca_base_url_data: env::var(alpaca_base_url_data).expect(&format!("Could not find: {}", alpaca_base_url_data)),
            alpaca_api_key: env::var(alpaca_api_key).expect(&format!("Could not find: {}", alpaca_api_key)),
            alpaca_api_secret: env::var(alpaca_api_secret).expect(&format!("Could not find: {}", alpaca_api_secret))
        }
    }    
}