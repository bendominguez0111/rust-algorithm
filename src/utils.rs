//utility functions for doing math
use apca::data::v2::last_quotes;

pub fn to_mid_prices(quotes: Vec<(String, last_quotes::Quote)>) -> Vec<(String, f64)> 
{
    return quotes
        .iter()
        .map(|(symbol, quote)| {
            let mid_price = (quote.bid_price.clone() + quote.ask_price.clone()).to_f64().unwrap() / 2.0;
            return (symbol.to_string(), mid_price)
        })
        .collect();
}

pub fn simple_moving_average(data: &[f64], window_size: usize) -> f64
{
    let data_len = data.len();
    if window_size > data_len {
        let window_size = data_len; // shadowing
        let window_sum: f64 = data.iter().sum();
        return window_sum / window_size as f64
    }
    else {
        let window_sum: f64 = data[data_len - window_size..].iter().sum();
        return window_sum / window_size as f64
    }
}
