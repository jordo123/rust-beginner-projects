/// The types of valid coins
pub enum CoinType {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
/// Estimates the amount of coins and the number of coin rolls needed
/// # Examples
/// ```
/// # use rust_beginner_projects::coin_estimator_by_weight::estimate;
/// # use rust_beginner_projects::coin_estimator_by_weight::CoinType;
/// let (amount_of_pennies, number_of_penny_rolls) = estimate(CoinType::Penny, 125.0);
/// assert_eq!(amount_of_pennies, 50);
/// assert_eq!(number_of_penny_rolls, 1);
/// ```
/// # Parameters
/// `coin_type` - The type of coin being used
///
/// `weight` - the total weight of the coins
/// # Preconditions
/// weight is positive
/// # Returns
/// `(i32, i32)` - The estimated amount of the coin and the number of coin rolls required
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
