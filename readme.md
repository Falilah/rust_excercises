1) ## `divisor()` Instruction
    function that computes the greatest common divisor of two integers, using [Euclid’s algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm). 


2) ## `calculate_cost()` Instruction

    Each car normally costs $10,000 to produce individually, regardless of whether it is successful or not. But with a bit of planning, 10 cars can be produced together for $95,000.

    For example, 37 cars can be produced in the following way: 37 = 3 x groups of ten + 7 individual cars

    So the cost for 37 cars is: 3*95,000+7*10,000=355,000

    Implement the function CalculateCost that calculates the cost of producing a number of cars, regardless of whether they are successful.


3) ## `prime_factors()` Instruction
    Compute the prime factors of a given natural number.

    A prime number is only evenly divisible by itself and 1.

    Note that 1 is not a prime number.

    Example
    What are the prime factors of 60?

    Our first divisor is 2. 2 goes into 60, leaving 30.
    2 goes into 30, leaving 15.
    2 doesn't go cleanly into 15. So let's move on to our next divisor, 3.
    3 goes cleanly into 15, leaving 5.
    3 does not go cleanly into 5. The next possible factor is 4.
    4 does not go cleanly into 5. The next possible factor is 5.
    5 does go cleanly into 5.
    We're left only with 1, so now, we're done.
    Our successful divisors in that computation represent the list of prime factors of 60: 2, 2, 3, and 5.


4) ## `nth_prime()` Instruction

    Given a number n, determine what the nth prime is.

    By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

    If your language provides methods in the standard library to deal with prime numbers, pretend they don't exist and implement them yourself.

    Remember that while people commonly count with 1-based indexing (i.e. "the 6th prime is 13"), many programming languages, including Rust, use 0-based indexing (i.e. primes[5] == 13). Use 0-based indexing for your implementation.