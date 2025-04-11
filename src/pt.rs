use std::math;

fn expo_decay(orderbook: Vec<Vec<f64>>, depth: f64) -> f64 {
    let mid = (orderbook[0][1] + orderbook [0][2]) / 2.0;
    let bo_spread = orderbook[0][2] - orderbook[0][1];
    let mut weighted_bid = 0.0;
    let mut weighted_ask = 0.0;
    for row in &orderbook {
        if (row[1] - mid).abs() < depth {
            weighted_bid += ((1/(row[1]-mid)/depth) * row[0]);
        }
        if (row[2] - mid).abs() < depth {
            weighted_ask += ((1/(row[2]-mid)/depth) * row[3]);
        }
    }
    let mut weighted_mid = orderbook[0][1] + (bo_spread * (weighted_bid / (weighted_bid + weighted_ask)));
    return weighted_mid;
}