use apca::Client;
use apca::api::v2::account;
use apca::api::v2::order;
use apca::api::v2::asset;
use crate::api::signals;
use num_decimal;

//make buy order given target allocation
pub async fn create_order(
    client: &Client,
    signal: &signals::SignalData
)
{   
    let account_info = get_account_info(&client).await;
    let cash_balance = account_info.cash.to_f64().unwrap();
    let notional = &signal.allocation * cash_balance;
    let closing_price = &signal.closing_price;

    //Converting to integers to do integer division
    let notional_int = notional as i32;
    let closing_price_int = *closing_price as i32; // dont understand why I have to use dereferncing here but it's what it wants me to do.
    let target_share_amount = notional_int / closing_price_int;

    //get current_share_amout here and find difference -> target share amount

    let order_request = order::OrderReqInit {
        class: order::Class::Simple,
        type_: order::Type::Market,
        ..Default::default()
    }.init(
        signal.symbol.clone(), signal.side, order::Amount::quantity(target_share_amount)
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