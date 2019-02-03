extern crate rust_beginner_projects;
use rust_beginner_projects::coin_estimator_by_weight;
use rust_beginner_projects::coin_estimator_by_weight::CoinType;

fn main() {
    const WEIGHT: f64 = 223.0;
    const TYPE: CoinType = CoinType::Quarter;
    let (amount, rolls) = coin_estimator_by_weight::estimate(TYPE, WEIGHT);
    println!("{} grams = {} with {} coin rolls", WEIGHT, amount, rolls);
}
