use apca::Client;
use apca::api::v2::account;
use apca::api::v2::order;
use apca::api::v2::asset;
use num_decimal;

//make buy order given target allocation
pub async fn create_order(
    client: &Client,
    ticker: &str,
    target_weighting: &f64
)
{   
    let account_info = get_account_info(&client).await;
    let cash_balance = account_info.cash.to_f64().unwrap();
    let notional = target_weighting * cash_balance;
    let notional_int = notional as i32;
    println!("Creating order for symbol {} for ${}", ticker, notional_int);

    let order_request = order::OrderReqInit {
        class: order::Class::Simple,
        type_: order::Type::Market,
        ..Default::default()
    }.init(
        ticker.to_string(), order::Side::Buy, order::Amount::notional(notional_int)
    );

    let order = client.issue::<order::Post>(&order_request).await.unwrap();
}

pub async fn get_account_info(
    client: &Client,
) -> account::Account
{
    let account = client.issue::<account::Get>(&()).await.unwrap();
    
    account
}

pub async fn get_current_allocation(
    client: &Client,
    ticker: &str
)
{
    println!("Working");
}