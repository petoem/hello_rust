use std::{thread, time};

fn main() {
    loop {
        println!("Hello, Rust!");
        thread::sleep(time::Duration::from_millis(1000));
    }
}
