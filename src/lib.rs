use isprime::check_prime;

fn prime_factors(n: u64) -> Vec<u64> {
    // todo!("This should calculate the prime factors of {n}")
    let mut all_prime = Vec::new();
    let mut num: u64 = n;
    loop{
    for i in 2..(num + 1){
        if check_prime(i) == true {
            if num % i == 0{
                num = num / i;
                all_prime.push(i);
                break;
            }else{
                continue
            }
        }
    }
    if num == 1{
        break;
    }
}
    all_prime
    
}

pub fn nth_prime(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")
    let mut count:u32 = 0;
    let mut index:u32 = 0;

    loop{
        if check_prime(index as u64){
            count += 1;
            if count - 1 == n{
                return index as u32;
            }
        }
        index +=1;
        
    }
}

// fn first_turn(card1:str, card2: str, dealer: str) -> str{
//     dealer
// }

pub fn verse(n: u32) -> String {
    // todo!("emit verse {n}")
    let mut v= String::new();
    if n == 0{
        v = format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    }
    else if n - 1 != 0{
        if n-1 == 1{
            v = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n , n , n -1);
        }
        else {
            v = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n , n , n -1);
        }
    }
    else{
     v = format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n);
     
    }
    v
    
}

pub fn sing(start: u32, end: u32) -> String {
    // todo!("sing verses {start} to {end}, inclusive")
    let mut poem = String::new();
    let val = start +1;
    for i in (end..val).rev()  {
        if i == end{
            poem.push_str(&format!("{}", verse(i)))
       }
       else{
       poem.push_str(&format!("{}\n", verse(i)));
       }
    }
     poem
}

pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")
    let has_alphabetic = message.chars().any(|c| c.is_alphabetic());
    let que: bool = message.chars().rev().find(|&c| !c.is_whitespace()).map_or(false, |c| c == '?');
    let yell = message.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());
    if que && yell && has_alphabetic{
        "Calm down, I know what I'm doing!"
    }
    else if que{
            "Sure."
         }
    else if (message.trim()).len() == 0{
        "Fine. Be that way!"
    }
     else if has_alphabetic && yell{
        "Whoa, chill out!"
    }
    else{
        "Whatever."
    }
}

fn main() {
    let string = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
    let arr: bool = brackets_are_balanced(string);
    println!("{:?}", arr);
}

pub fn brackets_are_balanced(string: &str) -> bool {
    // todo!("Check if the string \"{string}\" contains balanced brackets");
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
        }
        else if i > 1 && i + 1 % 2 != 0 {
            continue;
        }
        else if get_balance(ch, &i, &arr) {
            finalstate = true;

        } else {
            finalstate = false;
            
            break;
        }
    }

    finalstate
}

fn getbrs(string: &str) -> Vec<char>{
    string[0..]
        .chars()
        .filter(|&c| c == '{' || c == '}' || c == '(' || c == ')' || c == '[' || c == ']')
        .collect()
}

fn get_balance(ch: &char, i: &usize, br:&[char]) -> bool{
    match ch {
        &'{' => br[br.len() - (i + 1)] == '}' ||  br[i + 1] == '}',
        &'[' => br[br.len() - (i + 1)] == ']' ||  br[i + 1] == ']',
        &'(' => br[br.len() - (i + 1)] == ')' ||  br[i + 1] == ')',
        _ => false
    }
}






#[test]
fn test_prime_factor(){
    assert_eq!(prime_factors(100), [2, 2, 5, 5]);
    assert_eq!(prime_factors(60), [2, 2, 3, 5]);
    assert_eq!(prime_factors(75), [3, 5, 5]);



}

#[test]
fn test_nth_prime(){
    assert_eq!(nth_prime(10000), 104743);
    assert_eq!(nth_prime(60), 283);
    assert_eq!(nth_prime(7), 19);
    assert_eq!(nth_prime(5), 13);
}

#[test]
fn test_poem(){
    assert_eq!(verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");

    assert_eq!(sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");

    assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");

}
#[test]
fn test_reply(){
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