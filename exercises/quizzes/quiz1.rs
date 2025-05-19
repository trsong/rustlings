// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// Write a function that calculates the price of an order of apples given
// the quantity bought.

const APPLE_PRICE: u64 = 2;
const DISCOUNT_APPLE_PRICE: u64 = APPLE_PRICE / 2;

fn calculate_price_of_apples(num_apples: u64) -> u64 { 
    let unit_price = if num_apples > 40 {
        DISCOUNT_APPLE_PRICE
    } else {
        APPLE_PRICE
    };
    num_apples * unit_price
 }

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
