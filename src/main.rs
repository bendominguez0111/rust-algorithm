#![allow(warnings)]

mod config;

mod api {
    pub mod signals;
    pub mod stream;
    pub mod account;
}

pub mod utils;

use api::signals;
use api::stream;
use api::account;

use config::Environment;
use apca::Client;
use apca::ApiInfo;

// #[allow(unused_variables)]
#[tokio::main]
async fn main() {
    let paper_config = config::Config::load(Environment::Paper);
    //pairs trading algorithm goes here
    let api_info = ApiInfo::from_parts(
        paper_config.alpaca_base_url,
        paper_config.alpaca_api_key,
        paper_config.alpaca_api_secret
    ).unwrap();

    let client = Client::new(api_info);
    let tickers = config::universe();

    //get signal data for trading universe
    // let trading_signals = signals::get_signal_data(&client, &tickers).await;

    // for signal_obj in &trading_signals {
    //     match signal_obj.signal {
    //         signals::Signal::Buy => {
    //             println!("Submitting a buy order for {}", signal_obj.symbol);
    //         },
    //         signals::Signal::Sell => {
    //             println!("Submitting a sell order for {}", signal_obj.symbol);
    //         }
    //     }
    // }

    // account::create_order(&client, &"SPY", &0.10).await;
    let trading_signals = signals::get_signal_data(&client, &tickers).await;
    let current_aapl_allocation = account::get_current_allocation(&client, &"SPY").await;

}
