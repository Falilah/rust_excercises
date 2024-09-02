use isprime::check_prime;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use rand::{thread_rng, Rng};

//  3)

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
// 5)

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
 // 7)
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

// 9) 
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

// 10) 

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

// 11) 
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

// 12)
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

// 13)
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram: Vec<&str> = diagram.split_whitespace().collect();
    let position: (usize, usize) = get_position(&student[..1]);
    let mut result = Vec::new();
    for i in diagram {
        let x = &i[position.0..=position.1];
        result.push(get_plant(&x[..=0]));
        result.push(get_plant(&x[x.len() - 1..]));
    }
    result
}
fn get_position(student: &str) -> (usize, usize) {
    match student {
        "A" => (0, 1),
        "B" => (2, 3),
        "C" => (4, 5),
        "D" => (6, 7),
        "E" => (8, 9),
        "F" => (10, 11),
        "G" => (12, 13),
        "H" => (14, 15),
        "I" => (16, 17),
        "J" => (18, 19),
        "K" => (20, 21),
        "L" => (22, 23),
        _ => (0, 0),
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

// 14)
pub fn egg_count(display_value: u32) -> usize {
    // todo!("count the eggs in {display_value}")
    let bin = dec_to_bin(display_value);

    let mut count: usize = 0;
    for i in bin.chars() {
        if i == '1' {
            count += 1;
        }
    }
    count
}

fn dec_to_bin(mut display_value: u32) -> String {
    let mut bin = String::new();
    while display_value > 0 {
        let re = display_value % 2;
        bin.push_str(&re.to_string());

        display_value /= 2;
    }
    bin.chars().rev().collect()
}

#[derive(Debug)]
pub struct HighScores {
    score: Vec<u32>,
}


// 15)
impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut score: Vec<u32> = Vec::new();
        for &i in scores {
            score.push(i)
        }
        HighScores { score: score }
    }

    pub fn scores(&self) -> &[u32] {
        &self.score
    }

    pub fn latest(&self) -> Option<u32> {
        if self.score.is_empty() {
            return None;
        }

        let size = self.score.len();
        let data = self.score.get(size - 1).unwrap();
        Some(*data)
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.score.is_empty() {
            return None;
        }
        let mut personal_best = &0;
        for i in &self.score {
            if i > personal_best {
                personal_best = i;
            }
        }
        Some(*personal_best)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_scores = self.score.clone();
        top_scores.sort_unstable_by(|a, b| b.cmp(a));
        top_scores.into_iter().take(3).collect()
    }
}


// 16)
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
    let digits: Vec<&str> = code.split_whitespace().collect();
    let digits: Vec<char> = digits.join("").chars().collect();
    let mut sum = 0;
    for (i, dig) in digits.iter().rev().enumerate() {
        match dig.to_digit(10) {
            Some(digit) => {
                println!("{}: {}", i, dig);
                if i % 2 > 0 {
                    let mut val = digit * 2;
                    if val > 9 {
                        val -= 9;
                    }
                    sum += val;
                } else {
                    sum += digit;
                }
            }
            None => return false,
        }
    }

    sum % 10 == 0
}

// 17)
pub fn abbreviate(phrase: &str) -> String {
    let phrase = phrase.split_whitespace().collect::<Vec<&str>>();

    let mut acr: String = String::new();
    for i in phrase {
        let ch = &i[0..1].to_uppercase();
        if ch.chars().any(|c| c.is_alphabetic()) {
            acr.push_str(ch);
            let next = i[1..].chars().next();
            match next {
                Some(n) => {
                    if n.is_ascii_uppercase() {
                        continue;
                    }
                }
                None => continue,
            }
        }
        for (index, j) in i.chars().enumerate() {
            if index == 0 || j == '_' {
                continue;
            } else if j.is_ascii_uppercase() {
                acr.push_str(&i[index..index + 1]);
            } else if j == '-' {
                println!("{}", j);
                acr.push_str(&i[index + 1..index + 2].to_uppercase());
            }
        }
    }
    acr
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}


// 18)
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // todo!("Convert {number:?} from base {from_base} to base {to_base}");
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    } else if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    for i in number.iter() {
        if i >= &from_base {
            return Err(Error::InvalidDigit(*i));
        }
    }
    let ret = covert_to_b_10(number, from_base);
    println!("{:?}", ret);

    if to_base != 10 {
        Ok(convert_frm_b_10(&ret, to_base))
    } else {
        Ok(ret)
    }
}

fn covert_to_b_10(number: &[u32], from_base: u32) -> Vec<u32> {
    let mut sum = 0;
    for (i, b) in number.iter().rev().enumerate() {
        println!("{}", b);
        sum += b * (from_base.pow(i as u32));
    }
    println!("{}", sum);

    let ret = sum.to_string().chars().collect::<Vec<char>>();
    let mut n: Vec<u32> = Vec::new();

    for i in ret {
        n.push(i.to_digit(10).unwrap());
    }

    n
}
fn convert_frm_b_10(number: &Vec<u32>, to_base: u32) -> Vec<u32> {
    let mut res = combine(number);
    if res == 0 {
        return vec![0];
    }
    let mut tobase = Vec::new();
    while res > 0 {
        let rem = res % to_base;
        res /= to_base;
        tobase.push(rem);
    }
    tobase.reverse();
    println!("{:?}", tobase);
    tobase
}
fn combine(number: &Vec<u32>) -> u32 {
    let mut val = String::new();
    for i in number {
        val.push_str(i.to_string().as_str());
    }
    //    println!("{:?}", val);

    val.parse().unwrap()
}

#[derive(Debug, PartialEq, Eq)]

pub struct Allergies{
    score : u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies{score: score}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let all_in_score = self.allergies();
        for i in &all_in_score{
            if i == allergen{
                return true;
            }
        }
            return false    

    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;
        let mut all = Vec::new();
        let mut count  = 8;
        let mut x; 
        if self.score > 255{
            if score % 256 == 0{
            return  vec![Allergen::Eggs];
        } else{
            score = score - 256;
        }
         }       
            while count != 0{
                count -=1 ;
                 x = (2 as u32).pow(count);
                if score >= x{
                    all.push(score_to_allergies(x));
                    score -= x;
                    if score == 0{
                        break
                    }
                }
        }
        all.reverse();
        all
    }
}


fn score_to_allergies(score: u32) -> Allergen{
    match score {
        1 => Allergen::Eggs,
        2 =>Allergen::Peanuts,
        4 => Allergen:: Shellfish,
        8 => Allergen::Strawberries,
        16 => Allergen::Tomatoes,
        32 => Allergen::Chocolate,
        64 => Allergen::Pollen,
        _ => Allergen::Cats ,        
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
#[test]
fn test_egg_count() {
    let input = 0;
    let output = egg_count(input);
    let expected = 0;
    assert_eq!(output, expected);
    let input = 2_000_000_000;
    let output = egg_count(input);
    let expected = 13;
    assert_eq!(output, expected);
}

#[test]
fn test_high_scores() {
    let expected = [30, 50, 20, 70];
    let high_scores = HighScores::new(&expected);
    assert_eq!(high_scores.scores(), &expected);

    let high_scores = HighScores::new(&[40]);
    assert_eq!(high_scores.personal_top_three(), vec![40]);

    let high_scores = HighScores::new(&[]);
    assert_eq!(high_scores.latest(), None);

    let high_scores = HighScores::new(&[]);
    assert!(high_scores.personal_top_three().is_empty());
}

#[test]
fn test_is_valid() {
    assert!(is_valid("59"));
    assert!(!is_valid("59%59"));
    assert!(!is_valid(":9"));
    assert!(is_valid("095 245 88"));
    assert!(is_valid("234 567 891 234"));
    assert!(is_valid("9999999999 9999999999 9999999999 9999999999"));
}

#[test]
fn test_abbreviate() {
    let input = "First In, First Out";
    let output = abbreviate(input);
    let expected = "FIFO";
    assert_eq!(output, expected);

    let input = "Complementary metal-oxide semiconductor";
    let output = abbreviate(input);
    let expected = "CMOS";
    assert_eq!(output, expected);

    let input = "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me";
    let output = abbreviate(input);
    let expected = "ROTFLSHTMDCOALM";
    assert_eq!(output, expected);

    let input = "First In, First Out";
    let output = abbreviate(input);
    let expected = "FIFO";
    assert_eq!(output, expected);

    let input = "The Road _Not_ Taken";
    let output = abbreviate(input);
    let expected = "TRNT";
    assert_eq!(output, expected);
}

#[test]
fn test_convert() {
    let input_base = 2;
    let input_digits = &[1, 0, 1];
    let output_base = 10;
    let output_digits = vec![5];
    assert_eq!(
        convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );

    let input_base = 10;
    let input_digits = &[4, 2];
    let output_base = 2;
    let output_digits = vec![1, 0, 1, 0, 1, 0];
    assert_eq!(
        convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );

    let input_base = 16;
    let input_digits = &[2, 10];
    let output_base = 3;
    let output_digits = vec![1, 1, 2, 0];
    assert_eq!(
        convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );

    let input_base = 0;
    let input_digits = &[];
    let output_base = 10;
    assert_eq!(
        convert(input_digits, input_base, output_base),
        Err(Error::InvalidInputBase)
    );

    let input_base = 2;
    let input_digits = &[1, 2, 1, 0, 1, 0];
    let output_base = 10;
    assert_eq!(
        convert(input_digits, input_base, output_base),
        Err(Error::InvalidDigit(2))
    );

    let input_base = 2;
    let input_digits = &[1, 0, 1, 0, 1, 0];
    let output_base = 1;
    assert_eq!(
        convert(input_digits, input_base, output_base),
        Err(Error::InvalidOutputBase)
    );

    let input_base = 2;
    let input_digits = &[1, 0, 1, 0, 1, 0];
    let output_base = 10;
    let output_digits = vec![4, 2];
    assert_eq!(
        convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}


#[test]

fn test_allergies() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Peanuts));

    let allergies = Allergies::new(7);
    assert!(allergies.is_allergic_to(&Allergen::Peanuts));

    let allergies = Allergies::new(255);
    assert!(allergies.is_allergic_to(&Allergen::Shellfish));

    let allergies = Allergies::new(509).allergies();
    let expected = &[
        Allergen::Eggs,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    assert_eq!(&allergies, expected);

     let allergies = Allergies::new(257).allergies();
    let expected = &[Allergen::Eggs];
    assert_eq!(&allergies, expected);
}
