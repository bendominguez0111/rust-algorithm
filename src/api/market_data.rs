// wrapper for the alpaca market data API
use apca::Client;
use apca::data::v2::last_quotes;
use apca::data::v2::bars;
use apca::data::v2::Feed::IEX;

use chrono::{Utc, Duration};

use crate::utils;

pub async fn latest_mid_prices(client: &Client, tickers: Vec<String>) -> Vec<(String, f64)> 
{
    // get mid prices for a vec of tickers
    let req = last_quotes::LastQuotesReq {
        symbols: tickers,
        feed: Some(IEX)
    };
    let closing_prices = client.issue::<last_quotes::Get>(&req).await.unwrap();
    let mid_prices = utils::to_mid_prices(closing_prices);
    return mid_prices;
}

pub async fn get_bars(client: &Client, tickers: &Vec<&str>, n: i32) 
{
    for symbol in tickers {
        let req = bars::BarsReq {
            symbol: String::from(*symbol), // deferencing
            limit: Some(10_000),
            start: Utc::now() - Duration::days(30),
            end: Utc::now(),
            adjustment: Some(bars::Adjustment::Split), 
            timeframe: bars::TimeFrame::OneMinute,
            feed: Some(IEX),
            page_token: None
        };
        let bars = client.issue::<bars::Get>(&req).await.unwrap();
        let closing_prices: Vec<f64> = bars.bars.iter().map(|bar| bar.close.to_f64().unwrap()).collect();
        // sma 20
        let sma_20 = utils::simple_moving_average(&closing_prices, 20);
        // sma 8
        let sma_8 = utils::simple_moving_average(&closing_prices, 8);

        if (sma_8 > sma_20) {
            println!("Buy security: {}", symbol);
        } else {
            println!("Sell security: {}", symbol);
        }
    }
}