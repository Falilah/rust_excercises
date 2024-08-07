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

#[test]
fn test_prime_factor(){
    assert_eq!(prime_factors(100), [2, 2, 5, 5]);
    assert_eq!(prime_factors(60), [2, 2, 3, 5]);
    assert_eq!(prime_factors(75), [3, 5, 5]);



}