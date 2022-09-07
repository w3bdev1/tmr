use tmr::start_timer;

use std::{env, process::exit};

fn main() {
    let (min, sec) = parse_args(env::args().collect());
    start_timer(min, sec)
}

fn print_help() {
    println!("Usage: timer [[-m minutes] [-s seconds]] [-hv]")
}

fn print_version() {
    println!("Version 0.1")
}

fn parse_args(args: Vec<String>) -> (i32, i32) {
    let mut min = 0;
    let mut sec = 0;
    match args.len() {
        1 => {
            eprintln!("Too few args");
            print_help();
            exit(1)
        }
        x if x > 5 => {
            eprintln!("Too many args");
            print_help();
            exit(1)
        }
        2 => match &args[1][..] {
            "-h" => {
                print_help();
                exit(0)
            }
            "-v" => {
                print_version();
                exit(0)
            }
            _ => {
                eprintln!("Incorrect arg");
                print_help();
                exit(2)
            }
        },
        3 => {
            let (flag, time) = parse_time_arg(&args[1], &args[2]);
            match flag {
                'm' => min = time,
                's' => sec = time,
                _ => {}
            }
        }
        5 => {
            let option1 = &args[1];
            let num1 = &args[2];
            let option2 = &args[3];
            let num2 = &args[4];

            let (flag1, time1) = parse_time_arg(option1, num1);
            let (flag2, time2) = parse_time_arg(option2, num2);

            if flag1 == flag2 {
                eprintln!("Repeating same arg");
                print_help();
                exit(7)
            } else {
                match (flag1, flag2) {
                    ('m', 's') => {
                        min = time1;
                        sec = time2;
                    }
                    ('s', 'm') => {
                        min = time2;
                        sec = time1;
                    }
                    _ => {}
                }
            }
        }
        _ => {
            eprintln!("Incorrect argument combination");
            print_help();
            exit(8)
        }
    }
    (min, sec)
}

fn parse_time_arg(option: &str, num: &str) -> (char, i32) {
    let flag: char;
    let time: i32;
    match option {
        "-s" => flag = 's',
        "-m" => flag = 'm',
        x => {
            eprintln!("Incorrect arg `{}'", x);
            print_help();
            exit(3)
        }
    }
    match num.parse::<i32>() {
        Ok(n) => time = n,
        _ => {
            eprintln!("Non-numeric value given");
            print_help();
            exit(4)
        }
    };

    if flag == 's' && time >= 60 {
        eprintln!("Second out of bound (>59)");
        exit(5)
    }

    (flag, time)
}
