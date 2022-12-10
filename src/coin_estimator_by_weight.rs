
pub enum CoinType {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


pub fn estimate(coin_type: CoinType, weight: f64) -> (i32, i32) {
    match coin_type {
        CoinType::Penny => {
            let amount = (weight / 2.50).ceil();
            let rolls = (amount / 50.0).ceil() as i32;
            (amount as i32, rolls)
        }
        CoinType::Nickel => {
            let amount = (weight / 5.0).ceil();
            let rolls = (amount / 40.0).ceil() as i32;
            (amount as i32, rolls)
        }
        CoinType::Dime => {
            let amount = (weight / 2.268).ceil();
            let rolls = (amount / 50.0).ceil() as i32;
            (amount as i32, rolls)
        }
        CoinType::Quarter => {
            let amount = (weight / 5.67).ceil();
            let rolls = (amount / 40.0).ceil() as i32;
            (amount as i32, rolls)
        }
    }
}
