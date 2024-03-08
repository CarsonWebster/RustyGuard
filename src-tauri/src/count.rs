// count.rs
use std::{thread, time::Duration};

fn main() {
    for i in 1..=10 {
        println!("{}", i);
        thread::sleep(Duration::from_secs(1));
    }
}