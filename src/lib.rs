use isprime::check_prime;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use rand::{thread_rng, Rng};

//  1)

fn prime_factors(n: u64) -> Vec<u64> {
   
    let mut all_prime = Vec::new();
    let mut num: u64 = n;
    loop {
        for i in 2..(num + 1) {
            if check_prime(i) == true {
                if num % i == 0 {
                    num = num / i;
                    all_prime.push(i);
                    break;
                } else {
                    continue;
                }
            }
        }
        if num == 1 {
            break;
        }
    }
    all_prime
}

pub fn nth_prime(n: u32) -> u32 {
    let mut count: u32 = 0;
    let mut index: u32 = 0;

    loop {
        if check_prime(index as u64) {
            count += 1;
            if count - 1 == n {
                return index as u32;
            }
        }
        index += 1;
    }
}

// 6
pub fn verse(n: u32) -> String {
    let mut v = String::new();
    if n == 0 {
        v = format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else if n - 1 != 0 {
        if n - 1 == 1 {
            v = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n , n , n -1);
        } else {
            v = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n , n , n -1);
        }
    } else {
        v = format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n);
    }
    v
}

pub fn sing(start: u32, end: u32) -> String {
    
    let mut poem = String::new();
    let val = start + 1;
    for i in (end..val).rev() {
        if i == end {
            poem.push_str(&format!("{}", verse(i)))
        } else {
            poem.push_str(&format!("{}\n", verse(i)));
        }
    }
    poem
}

pub fn reply(message: &str) -> &str {
    let has_alphabetic = message.chars().any(|c| c.is_alphabetic());
    let que: bool = message
        .chars()
        .rev()
        .find(|&c| !c.is_whitespace())
        .map_or(false, |c| c == '?');
    let yell = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase());
    if que && yell && has_alphabetic {
        "Calm down, I know what I'm doing!"
    } else if que {
        "Sure."
    } else if (message.trim()).len() == 0 {
        "Fine. Be that way!"
    } else if has_alphabetic && yell {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let arr: Vec<char> = getbrs(string);
    println!("{:?}", arr);

    if arr.len() % 2 > 0 {
        return false;
    } else if arr.is_empty() {
        return true;
    }
    let mut finalstate = false;

    for (i, ch) in arr.iter().enumerate() {
        println!("{}", ch);

        if i + 1 == arr.len() / 2 && arr.len() != 2 || i == arr.len() - 1 {
            break;
        } else if i > 1 && i + 1 % 2 != 0 {
            continue;
        } else if get_balance(ch, &i, &arr) {
            finalstate = true;
        } else {
            finalstate = false;

            break;
        }
    }

    finalstate
}

fn getbrs(string: &str) -> Vec<char> {
    string[0..]
        .chars()
        .filter(|&c| c == '{' || c == '}' || c == '(' || c == ')' || c == '[' || c == ']')
        .collect()
}

fn get_balance(ch: &char, i: &usize, br: &[char]) -> bool {
    match ch {
        &'{' => br[br.len() - (i + 1)] == '}' || br[i + 1] == '}',
        &'[' => br[br.len() - (i + 1)] == ']' || br[i + 1] == ']',
        &'(' => br[br.len() - (i + 1)] == ')' || br[i + 1] == ')',
        _ => false,
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut n: u64 = n;
    let mut counter: u64 = 0;
    while n > 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = (n * 3) + 1;
        }
        counter += 1;
    }
    Some(counter)
}
pub fn private_key(p: u64) -> u64 {
    if !check_prime(p) {
        panic!("Not a prime numer")
    }
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let g = BigUint::from_u64(g).unwrap();
    let p = BigUint::from_u64(p).unwrap();
    let a = BigUint::from_u64(a).unwrap();

    let result = g.modpow(&a, &p);

    result.try_into().unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let b_pub: BigUint = BigUint::from_u64(b_pub).unwrap();
    let p = BigUint::from_u64(p).unwrap();
    let a = BigUint::from_u64(a).unwrap();

    let result = b_pub.modpow(&a, &p);

    result.try_into().unwrap()
}

pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();
    let size = digits.len();
    if size >= len {
        for i in 0..size {
            let val = &digits[i..(len + i)];
            result.push(String::from(val));

            if i + len == size {
                break;
            }
        }
    }
    result
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram: Vec<&str> = diagram.split_whitespace().collect();
    let position: (usize, usize) = get_position(&student[..1]);
    let mut result = Vec::new();
    for i in diagram{
        let x = &i[position.0 ..=  position.1];
        result.push(get_plant(&x[..=0]));
        result.push(get_plant(&x[x.len() - 1..]));
    }
    result



}
fn get_position(student: &str) -> (usize, usize)  {
    match student {
        "A" => (0, 1),
        "B" => (2,3),
        "C" => (4,5),
        "D" => (6, 7),
        "E" => (8,9),
        "F" => (10, 11),
        "G" => (12, 13),
        "H" => (14, 15),
        "I" => (16, 17),
        "J" => (18, 19),
        "K" => (20, 21),
        "L" => (22, 23),
        _ => (0,0)
    }
}


    fn get_plant(plant: &str) -> &'static str {
        match plant {
            "G" => "grass",
            "C" => "clover",
            "R" => "radishes",
            "V" => "violets",
            _ => "",
        }


}

#[test]
fn test_prime_factor() {
    assert_eq!(prime_factors(100), [2, 2, 5, 5]);
    assert_eq!(prime_factors(60), [2, 2, 3, 5]);
    assert_eq!(prime_factors(75), [3, 5, 5]);
}

#[test]
fn test_nth_prime() {
    assert_eq!(nth_prime(10000), 104743);
    assert_eq!(nth_prime(60), 283);
    assert_eq!(nth_prime(7), 19);
    assert_eq!(nth_prime(5), 13);
}

#[test]
fn test_poem() {
    assert_eq!(verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");

    assert_eq!(sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");

    assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
}
#[test]
fn test_reply() {
    assert_eq!(reply("WATCH OUT!"), "Whoa, chill out!");
    assert_eq!(reply("4?"), "Sure.");
    assert_eq!(reply("1, 2, 3"), "Whatever.");
    assert_eq!(reply(":) ?"), "Sure.");
    assert_eq!(reply("          "), "Fine. Be that way!");
}

#[test]

fn test_brackets_are_balanced() {
    assert!(!brackets_are_balanced("[["));
    assert!(!brackets_are_balanced("}{"));

    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
    \\end{array}\\right)";
    assert!(brackets_are_balanced(input));

    assert!(!brackets_are_balanced("{]"));
    assert!(brackets_are_balanced(""));
}

#[test]

fn test_collatz() {
    let output = collatz(1);
    let expected = Some(0);
    assert_eq!(output, expected);

    let output = collatz(16);
    let expected = Some(4);
    assert_eq!(output, expected);

    let output = collatz(1_000_000);
    let expected = Some(152);
    assert_eq!(output, expected);

    let output = collatz(0);
    let expected = None;
    assert_eq!(output, expected);
}

#[test]

fn test_secret() {
    let p: u64 = 13;
    let g: u64 = 11;

    let private_key_a = private_key(p);
    let private_key_b = private_key(p);

    let public_key_a = public_key(p, g, private_key_a);
    let public_key_b = public_key(p, g, private_key_b);

    // Key exchange
    let secret_a = secret(p, public_key_b, private_key_a);
    let secret_b = secret(p, public_key_a, private_key_b);
    assert_eq!(secret_a, secret_b);
}

#[test]
fn test_series() {
    let input = "1";
    let length = 1;
    let expected = &["1"];
    assert_eq!(series(input, length), expected);
    let input = "777777";
    let length = 3;
    let expected = &["777", "777", "777", "777"];
    assert_eq!(series(input, length), expected);

    let input = "918493904243";
    let length = 5;
    let output = series(input, length);
    let expected = &[
        "91849", "18493", "84939", "49390", "93904", "39042", "90424", "04243",
    ];
    assert_eq!(output, expected);

    let input = "12345";
    let length = 6;
    let output = series(input, length);
    let expected: &[&str] = &[];
    assert_eq!(output, expected);
}
