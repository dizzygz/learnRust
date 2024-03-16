pub mod stocks;

use serde_json::{json, Value};
use stocks::structs::stock::Stock;
/// Adds two numbers together.

///

/// # Arguments

/// * one (i32): one of the numbers to be added

/// * two (i32): one of the numbers to be added

///

/// # Returns

/// (i32): the sum of param one and param two

///

/// # Usage

/// The function can be used by the following code:

///

/// '''rust

/// result: i32 = add_numbers(2, 5);

/// '''

fn add_numbers(one: i32, two: i32) -> i32 {
    return one + two;
}

fn main() {
    let stock: Value = json!({

        "name": "MonolithAi",

        "price": 43.7,

        "history": [19.4, 26.9, 32.5]

    });

    println!("first price: {}", stock["history"][0]);

    println!("{}", stock.to_string());

    let stock: Stock = Stock::new("MonolithAi", 36.5);

    println!("here is the stock name: {}", stock.name);

    println!("here is the stock name: {}", stock.current_price);
}
