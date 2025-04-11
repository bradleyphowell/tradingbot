use rand::Rng;
use std::f64::consts::PI;

//Creates an orderbook of depth ob_size with a bid and offer price and volume ordered as follows:
// [bid volume, bid price, offer price, offer volume]
// The orderbook is a vector of vectors, where each inner vector represents an order in the orderbook.

fn make_ob(ob_size: i32, bid: f32, offer: f32) -> Vec<Vec<f64>> {
    let mut orderbook = Vec::new();
    let price_increment = 0.1;
    for i in 0..ob_size {
        let mut rng = rand::thread_rng();
        let mut pb = bid - (i * price_increment);
        let mut po = offer + (i * price_increment);
        let mut vb = rng.gen_range(1.0..100.0);
        let mut vo = rng.gen_range(1.0..100.0);
        let mut order = vec![vb as f32, pb as f64, po as f64, vo as f32];
        orderbook.push(order);
    }
    orderbook
}