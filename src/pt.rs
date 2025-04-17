// Function to calculate the orderbook imbalance
// this is done by applying an eponential decay to the weighting of volume placed further away from
// the midprice.
/* 
pub fn expo_decay(orderbook: Vec<Vec<f64>>, depth: f64) -> f64 {
    let mid = (orderbook[0][1] + orderbook [0][2]) / 2.0;
    let bo_spread = orderbook[0][2] - orderbook[0][1];
    let mut weighted_bid = 0.0;
    let mut weighted_ask = 0.0;
    for row in &orderbook {
        if ((row[1] - mid)/mid).abs() < depth {
            weighted_bid += ((1.0/(mid-row[1])/depth) * row[0]);
        }
        if ((row[2] - mid)/mid).abs() < depth {
            weighted_ask += ((1.0/(row[2]-mid)/depth) * row[3]);
        }
    }
    //println!("weighted bid is {}", weighted_bid);
    //println!("weighted ask is {}", weighted_ask);
    //println!("The bid offer spread is {}", bo_spread);
    let orderbook_skew = (weighted_bid/(weighted_bid + weighted_ask));
    let weighted_mid = orderbook[0][1] + (bo_spread * (weighted_bid / (weighted_bid + weighted_ask)));
    //println!("weighted mid is {}", weighted_mid);
    return orderbook_skew;
}

*/