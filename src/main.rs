// A simple program to test kaprekars constant 

use rand::Rng;


fn main() {
    println!("Kaprekar's Constant demo in Rust");
    
    // generate random 4 digit number
    let rng = rand::random_range(1000..9999);
    let it = kaprekar_step(rng);
    println!("{rng} took {it} steps to reach 6174");
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