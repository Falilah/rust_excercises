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