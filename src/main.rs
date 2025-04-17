use std::vec;

mod order_books {
    pub mod ob_gen;
}
mod pt;
mod inventory;
mod config;
mod data_collector;

struct Trade {
    side: String,
    size: f64,
    price: f64, 
}

#[tokio::main]
async fn main() {

    data_collector::connect_to_kraken().await;

/************Testing module ob_gen.rs works and provides accurate orderbooks ****************\ 
 
    let orderbook: Vec<Vec<f64>> = order_books::ob_gen::make_ob(10, 100.0, 101.0);

    for row in &orderbook {
        println!("{:?}", row);
    }

    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
    let orderbook2: Vec<Vec<f64>> = order_books::ob_gen::make_ob(10, 100.5, 101.0);

    for row in &orderbook2 {
        println!("{:?}", row);
    }
    
\********************************************************************************************/

//let orderbook: Vec<Vec<f64>> = order_books::ob_gen::make_ob(50, 100.0, 101.0);

//let ob_vol_weight = pt::expo_decay(orderbook, 0.02);

//let (mut base_assets, mut quote_assets) = inventory::inventory_balance(base_assets, quote_assets, &trade);

//let inventory_skew: f64 = 0.5 + ((config::START_BASE_INVENTORY - base_assets)/(config::RISK_LIMIT * 2.0));


/*********************************Testing the orderbook skew*********************************\

    let orderbook: Vec<Vec<f64>> = order_books::ob_gen::make_ob(100, 100.0, 101.0);

    let ob_vol_weight = pt::expo_decay(orderbook, 0.02);
    println!("The orderbook volume weighted mid is {}", ob_vol_weight);
}

\********************************************************************************************/

/*********************************Testing the inventory skew**********************************\
    let mut base_assets = 100.0;
    let mut quote_assets = 10000.0;

    let trade: Trade = Trade {
        side: String::from("buy"),
        size: 5.0,
        price: 100.0,
    };


    //Calculates inventory skew as a linear relationship to exposure with 0 representing max long  
    //exposure so bots should offered and 1 representing max short exposure so bots should be bid.
    let (mut base_assets, mut quote_assets) = inventory::inventory_balance(base_assets, quote_assets, &trade);
    
    let x: f64 = (config::START_BASE_INVENTORY - base_assets)/(config::RISK_LIMIT * 2.0);
    let inventory_skew = 0.5 + x;  
    println!("Base assets: {}", base_assets);
    println!("Quote assets: {}", quote_assets);
    println!("Inventory skew: {}", inventory_skew);  

\********************************************************************************************/

//let mut bidness = bidness_calc(inventory_skew, ob_vol_weight, of_balance, config::A, config::B, config::C);


}

/***************************************************************************************************************\

fn bidness_calc(inventory_skew: f64, ob_vol_weight: f64, of_balance: f64, A: f64, B: f64, C: f64) -> f64 {

    let mut bidness = 0.0;


}

\***************************************************************************************************************/