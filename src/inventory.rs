use crate::Trade;




pub fn inventory_balance(bq: f64, qq: f64, t_details: &Trade) -> (f64, f64) {

    if t_details.side == "buy" {
        let new_bq = bq + t_details.size;
        let new_qq = qq - t_details.size * t_details.price;
        return (new_bq, new_qq);
    } else if t_details.side == "sell" {
        let new_bq = bq - t_details.size;
        let new_qq = qq + t_details.size * t_details.price;
        return (new_bq, new_qq);
    } else {
        panic!("Invalid trade side: {}", t_details.side);
    }

}