// A simple program to test kaprekars constant

use std::io::{Write, stdin, stdout};

fn main() {
    println!("Kaprekar's Constant demo in Rust");

    // ask the user how many times to run
    let count = prompt_number("How many numbers would you like to try".to_string());

    for _ in 0..count {
        // generate random 4 digit number
        let rng = rand_number();
        println!("Testing {rng}");
        let it = kaprekar_step(rng);
        println!("{rng} took {it} steps to reach 6174");
    }
}

fn kaprekar_step(n: u32) -> u32 {
    let mut num = n;

    let mut it = 0;
    while num != 6174 {
        let lg = digit_sort_lg(num);
        let sm = digit_sort_sm(num);

        num = lg - sm;
        it += 1;
    }
    return it;
}

fn digit_sort_sm(n: u32) -> u32 {
    let mut v = num_to_vec(n);
    v.sort();

    // convert vec back to number
    let t = v.iter().fold("".to_string(), |acc, x| acc + &x.to_string());
    return t.parse::<u32>().unwrap();
}

fn digit_sort_lg(n: u32) -> u32 {
    let mut v = num_to_vec(n);
    v.sort();
    v.reverse();

    // convert vec back to number
    let t = v.iter().fold("".to_string(), |acc, x| acc + &x.to_string());
    return t.parse::<u32>().unwrap();
}

fn num_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn prompt_number(msg: String) -> i32 {
    let mut is_valid = false;
    let mut num = 0;

    let mut s = String::new();

    while !is_valid {
        println!("{msg} ");

        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a string");

        // validate input
        num = match s.trim().parse() {
            Ok(val) => {
                is_valid = true;
                val
            }
            Err(err) => {
                println!("Invalid String: {err}");
                0
            }
        };
    }
    return num;
}

fn rand_number() -> u32 {
    loop {
        let n = rand::random_range(1000..9999);
        let digits: Vec<char> = n.to_string().chars().collect();
        let unique_digits: std::collections::HashSet<char> = digits.iter().cloned().collect();
        if unique_digits.len() > 2 {
            return n;
        }
    }
}
