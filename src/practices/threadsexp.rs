use std::{thread, time};
use std::thread::JoinHandle;

pub fn simple_thread(seconds: i8, name:&str)-> i8 {
    println!("Thread {} is running", name);
    let totoal_seconds = time::Duration::from_secs_f32(seconds as f32);
    thread::sleep(totoal_seconds);
    println!("Thread {} is finished", name);
    seconds
}