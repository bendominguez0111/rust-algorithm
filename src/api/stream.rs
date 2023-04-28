use apca::Client;

use apca::Error;

use apca::data::v2::stream::RealtimeData;
use apca::data::v2::stream::IEX;
use apca::data::v2::stream::Bar;
use apca::data::v2::stream::Quote;
use apca::data::v2::stream::Trade;
use apca::data::v2::stream::MarketData;
use apca::data::v2::stream::Symbols;
use apca::data::v2::stream::SymbolList;
use apca::data::v2::stream::Data;

use apca::data::v2::stream::drive;

use futures::FutureExt as _;
use futures::StreamExt as _;
use futures::TryStreamExt as _;

pub async fn stream_quotes(client: &Client, tickers: Vec<String>) {
    println!("Grabbing data for: {:?}", tickers);
    let (mut stream, mut subscription) = client.subscribe::<RealtimeData<IEX>>().await.unwrap();
    let mut data = MarketData::default();
    data.set_quotes(tickers);
    let subscribe = subscription.subscribe(&data).boxed_local().fuse();
    let () = drive(subscribe, &mut stream)
        .await
        .unwrap()
        .unwrap()
        .unwrap();

    let () = stream
        .take(100000)
        .map_err(Error::WebSocket)
        .try_for_each(|result| async {
            result.map(|data| { // apca::v2::stream::Data enum
                match data {
                    Data::Quote(q) => {
                        let mid_price = (q.bid_price + q.ask_price)/2;
                        println!("{} mid price: {}", q.symbol, mid_price);
                    }
                    _ => {
                        println!("Received non-bar");
                    }
                }
            }).map_err(Error::Json)
        })
        .await
        .unwrap();
}

//streaming data
pub async fn stream_bars(client: &Client, tickers: Vec<String>) {
    println!("Grabbing data for: {:?}", tickers);
    let (mut stream, mut subscription) = client.subscribe::<RealtimeData<IEX>>().await.unwrap();
    let mut data = MarketData::default();
    data.set_bars(tickers);
    let subscribe = subscription.subscribe(&data).boxed_local().fuse();
    let () = drive(subscribe, &mut stream)
        .await
        .unwrap()
        .unwrap()
        .unwrap();

    let () = stream
        .take(10000)
        .map_err(Error::WebSocket)
        .try_for_each(|result| async {
            result.map(|data| { // apca::v2::stream::Data enum
                match data {
                    Data::Bar(bar) => {
                        println!("{} close price: {}", bar.symbol, bar.close_price);
                    }
                    _ => {
                        println!("Received non-quote");
                    }
                }
            }).map_err(Error::Json)
        })
        .await
        .unwrap();
}