// wrapper for the alpaca market data API
use apca::Client;
use apca::data::v2::last_quotes;
use apca::data::v2::bars;
use apca::data::v2::Feed::IEX;
use apca::api::v2::order;

use futures::future::join_all;
use chrono::{Utc, Duration};
use crate::utils;

#[derive(Debug)]
pub struct SignalData {
    pub symbol: String,
    pub closing_price: f64,
    pub standard_dev: f64,
    pub short_ma: f64,
    pub long_ma: f64,
    pub spread: f64,
    pub side: order::Side,
    pub allocation: f64
}

pub async fn get_signal_data(
    client: &Client, 
    tickers: &Vec<&str>
) -> Vec<SignalData>
{
    let futures = tickers.iter().map(|symbol| async move {

        let req = bars::BarsReq {
            symbol: String::from(*symbol), // deferencing
            limit: Some(10_000),
            start: Utc::now() - Duration::days(5),
            end: Utc::now(),
            adjustment: Some(bars::Adjustment::Split), 
            timeframe: bars::TimeFrame::OneMinute,
            feed: Some(IEX),
            page_token: None
        };

        let bars = client.issue::<bars::Get>(&req).await.unwrap();
        let closing_prices: Vec<f64> = bars.bars.iter().map(|bar| bar.close.to_f64().unwrap()).collect();
        let percentage_returns: Vec<f64> = utils::pct_change(&closing_prices);

        let short_ma = utils::simple_moving_average(&closing_prices, 10);
        let long_ma = utils::simple_moving_average(&closing_prices, 20);

        let last_closing_price = closing_prices.last().unwrap();
        let standard_dev = utils::std_dev(&percentage_returns);

        let mut side = order::Side::Buy;
        if (short_ma < long_ma) {
            side = order::Side::Sell;
        }

        let mut spread = short_ma - long_ma;
        spread = spread.abs();

        return SignalData {
            symbol: symbol.to_string(),
            closing_price: *last_closing_price,
            standard_dev: standard_dev,
            short_ma: short_ma,
            long_ma: long_ma,
            spread: spread,
            side: side,
            allocation: 0.10 // some initialization
        };

    });

    let mut results = join_all(futures).await;

    //create allocations here
    //not accounting for volatility yet
    //weight by signal / closing price
    //once we account for volatility we'll weight by signal / standard deviation
    let total_weighting: f64 = results.iter().map(|signal| {
        return (signal.spread / signal.standard_dev);
    }).sum();

    for signal in &mut results {
        signal.allocation = (signal.spread / signal.standard_dev) / total_weighting;
    }

    results
}