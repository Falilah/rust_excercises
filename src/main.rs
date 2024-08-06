use std::str::FromStr;
use std::env;

fn divisor(mut m: u64, mut n:u64) -> u64{
    assert!(m != 0 && n != 0);
    while m != 0{
        if m < n{
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n

}
fn calculate_cost(qty: u64) -> u64{
    let group:u64 = 95000;
    let single:u64 = 10000;
    let cost: u64;
    if qty > 10{
        let g = qty/10;
        let rem  = qty % 10;
        cost = (g * group) + (rem * single);
    }
    else{
        cost = qty * single;
    }
    cost
}

fn main() {
    let mut nums = Vec::new();

    for val in env::args().skip(1){
        nums.push(u64::from_str(&val).expect("Error parsing argument"));

    }

    if nums.len() == 0{
        eprintln!("Invalid argument count...")
    }
    let mut d = nums[0];

    for m in &nums[1..]{
        d = divisor(*m, d);
    }

    println!("the greatest divisor of {:?} is {}", nums, d);
    println!("the cost of 500 is: {}", calculate_cost(500));
    
}
#[test]
fn test_cost() {
    assert_eq!(calculate_cost(37), 355000);

}

#[test]
fn test_divisor(){
    assert_eq!(divisor(1*3*5*6, 2*8*9), 18);

}