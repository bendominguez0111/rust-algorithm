// wrapper for the alpaca market data API
use apca::Client;
use apca::data::v2::last_quotes;
use apca::data::v2::Feed::IEX;

fn to_mid_prices(quotes: Vec<(String, last_quotes::Quote)>) -> Vec<(String, f64)> {
    return quotes
        .iter()
        .map(|(symbol, quote)| {
            let mid_price = (quote.bid_price.clone() + quote.ask_price.clone()).to_f64().unwrap() / 2.0;
            return (symbol.to_string(), mid_price)
        })
        .collect();
}

pub async fn latest_mid_prices(client: &Client, tickers: Vec<String>) -> Vec<(String, f64)> {
    // get mid prices for a vec of tickers
    let req = last_quotes::LastQuotesReq {
        symbols: tickers,
        feed: Some(IEX)
    };
    let closing_prices = client.issue::<last_quotes::Get>(&req).await.unwrap();
    let mid_prices = to_mid_prices(closing_prices);
    return mid_prices;
}