use std::{io::{stdout, Write}, thread::sleep};
use std::time::Duration;

pub fn start_timer(mut min: i32, mut sec: i32) {
    while min >= 0 && sec >= 0 {
        print!("Timer: {:0>2}:{:0>2}", min, sec);
        stdout().flush().expect("Could not flush stdout");
        if sec == 0 {
            min -= 1;
            sec = 59;
        } else {
            sec -= 1;
        }
        sleep(Duration::from_secs(1));
        print!("\r");
    }
    println!("Timer: Time up!");
}
