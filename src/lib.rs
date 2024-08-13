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
       let size = message.len();

    let que = &message[size -1..] == "?";
    let yell = message.chars().all(|c| !c.is_lowercase());
    if  que && yell{
        "Calm down, I know what I'm doing!"
    }
    else if que{
        "Sure." 
    }
          else if message.trim().len() == 0{
        "Fine. Be that way!"
    }
    else if yell{
        "Whoa, chill out!"
    }
    else{
        "Whatever."
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