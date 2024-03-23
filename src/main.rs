pub mod practices;
pub mod stocks;

use serde_json::{json, Value};
use stocks::structs::stock::Stock;

use stocks::{close_order, open_order};

use practices::threadsexp::simple_thread;
use std::thread;
use stocks::structs::order::Order;

use std::thread::JoinHandle;

use std::{env, time};
use stocks::enums::order_types::OrderType;

use rand::prelude::*;

use std::str::FromStr;

use std::any::Any;

use std::marker::Send;

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

fn process_thread(thread_result: Result<i8, Box<dyn Any + Send>>, name: &str) {
    match thread_result {
        Ok(result) => {
            println!("the result for {} is {}", result, name);
        }

        Err(result) => {
            if let Some(string) = result.downcast_ref::<String>() {
                println!("the error for {} is: {}", name, string);
            } else {
                println!("there error for {} does not have a message", name);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let action: &String = &args[1];

    let name: &String = &args[2];

    let amount: i32 = i32::from_str(&args[3]).unwrap();

    let price: f32 = f32::from_str(&args[4]).unwrap();
    let mut new_order: Order =
        open_order(amount, OrderType::Long, &name.as_str(), price, None, None);

    let stock: Value = json!({

        "name": "MonolithAi",

        "price": 43.7,

        "history": [19.4, 26.9, 32.5]

    });

    match action.as_str() {
        "buy" => {
            println!(
                "the value of your investment is: {}",
                new_order.current_value()
            );
        }

        "sell" => {
            let mut rng = rand::thread_rng();

            let new_price_ref: f32 = rng.gen();

            let new_price: f32 = new_price_ref * 100 as f32;

            new_order.stock.update_price(new_price);

            let sale_profit: f32 = close_order(new_order);

            println!("here is the profit you made: {}", sale_profit);
        }

        _ => {
            panic!("Only 'buy' and 'sell' actions are supported");
        }
    }

    println!("first price: {}", stock["history"][0]);

    println!("{}", stock.to_string());

    let stock: Stock = Stock::new("MonolithAi", 36.5);

    println!("here is the stock name: {}", stock.name);

    println!("here is the stock name: {}", stock.current_price);

    println!("hello stocks");

    // let mut new_order: Order = open_order(20, OrderType::Long, "bumper", 56.8, None, None);

    // println!("the current price is: {}", &new_order.current_value());

    // println!("the current profit is: {}", &new_order.current_profit());
    // new_order.stock.update_price(43.1);

    // println!("the current price is: {}", &new_order.current_value());

    // println!("the current profit is: {}", &new_order.current_profit());

    // new_order.stock.update_price(82.7);

    // println!("the current price is: {}", &new_order.current_value());

    // println!("the current profit is: {}", &new_order.current_profit());
    // let profit: f32 = close_order(new_order);

    // println!("we made {} profit", profit);
    let base_rate: f32 = 0.03;

    let calculate_interest = |loan_amount: &f32| return loan_amount * &base_rate;

    println!(
        "the total interest to be paid is: {}",
        calculate_interest(&32567.6)
    );

    let now = time::Instant::now();
    let thread1: JoinHandle<i8> = thread::spawn(|| simple_thread(5, "one"));
    let thread2: JoinHandle<i8> = thread::spawn(|| simple_thread(5, "two"));
    let thread3: JoinHandle<i8> = thread::spawn(|| simple_thread(5, "three"));

    let result_one = thread1.join();

    let result_two = thread2.join();

    let result_three = thread3.join();

    println!("time elapsed {:?}", now.elapsed());

    process_thread(result_one, "one");

    process_thread(result_two, "two");

    process_thread(result_three, "three");
}
