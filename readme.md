1. ## `divisor()` Instruction

   function that computes the greatest common divisor of two integers, using [Euclid’s algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).

2. ## `calculate_cost()` Instruction

   Each car normally costs $10,000 to produce individually, regardless of whether it is successful or not. But with a bit of planning, 10 cars can be produced together for $95,000.

   For example, 37 cars can be produced in the following way: 37 = 3 x groups of ten + 7 individual cars

   So the cost for 37 cars is: 3*95,000+7*10,000=355,000

   Implement the function CalculateCost that calculates the cost of producing a number of cars, regardless of whether they are successful.

3. ## `prime_factors()` Instruction

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

4. ## `nth_prime()` Instruction

   Given a number n, determine what the nth prime is.

   By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

   If your language provides methods in the standard library to deal with prime numbers, pretend they don't exist and implement them yourself.

   Remember that while people commonly count with 1-based indexing (i.e. "the 6th prime is 13"), many programming languages, including Rust, use 0-based indexing (i.e. primes[5] == 13). Use 0-based indexing for your implementation.

5. ## `Blackjack_game()` Instruction => TODO!

    <img src="./blackjack.jpeg" alt="Alt text" width="300"/>

   Note: Commonly, aces can take the value of 1 or 11 but for simplicity we will assume that they can only take the value of 11.

   Depending on your two cards and the card of the dealer, there is a strategy for the first turn of the game, in which you have the following options:

   Stand (S)

   Hit (H)

   Split (P)

   Automatically win (W)

   Although not optimal yet, you will follow the strategy your friend Alex has been developing, which is as follows:

   - If you have a pair of aces you must always split them.
   - If you have a Blackjack (two cards that sum up to a value of 21), and the dealer does not have an ace, a figure or a ten then you automatically win. If the dealer does have any of those cards then you'll have to stand and wait for the reveal of the other card.
   - If your cards sum up to a value within the range [17, 20] you should always stand.
   - If your cards sum up to a value within the range [12, 16] you should always stand unless the dealer has a 7 or higher, in which case you should always hit.
   - If your cards sum up to 11 or lower you should always hit.

6. ## Beer song `verse()` and `sing()` Instruction
   Recite the lyrics to that beloved classic, that field-trip favorite: 99 Bottles of Beer on the Wall.

example:

    3 bottles of beer on the wall, 3 bottles of beer.
    Take one down and pass it around, 2 bottles of beer on the wall.

    2 bottles of beer on the wall, 2 bottles of beer.
    Take one down and pass it around, 1 bottle of beer on the wall.

    1 bottle of beer on the wall, 1 bottle of beer.
    Take it down and pass it around, no more bottles of beer on the wall.

    No more bottles of beer on the wall, no more bottles of beer.
    Go to the store and buy some more, 99 bottles of beer on the wall.

7. ## `reply()` Instruction

   Bob is a lackadaisical teenager. He likes to think that he's very cool. And he definitely doesn't get excited about things. That wouldn't be cool.

   When people talk to him, his responses are pretty limited.

   Instructions
   Your task is to determine what Bob will reply to someone when they say something to him or ask him a question.

   Bob only ever answers one of five things:

   - "Sure." This is his response if you ask him a question, such as "How are you?" The convention used for questions is that it ends with a question mark.
   - "Whoa, chill out!" This is his answer if you YELL AT HIM. The convention used for yelling is ALL CAPITAL LETTERS.
   - "Calm down, I know what I'm doing!" This is what he says if you yell a question at him.
   - "Fine. Be that way!" This is how he responds to silence. The convention used for silence is nothing, or various combinations of whitespace characters.
   - "Whatever." This is what he answers to anything else.

8. ## classic Frogger game Instruction => TODO!

   Manage a game player's High Score list.

   Your task is to build a high-score component of the classic Frogger game, one of the highest selling and most addictive games of all time, and a classic of the arcade era. Your task is to write methods that return the highest score from the list, the last added score and the three highest scores.

9. ## Matching Bracket `brackets_are_balanced()` Instruction

   **Introduction**
   You're given the opportunity to write software for the Bracketeer™, an ancient but powerful mainframe. The software that runs on it is written in a proprietary language. Much of its syntax is familiar, but you notice lots of brackets, braces and parentheses. Despite the Bracketeer™ being powerful, it lacks flexibility. If the source code has any unbalanced brackets, braces or parentheses, the Bracketeer™ crashes and must be rebooted. To avoid such a scenario, you start writing code that can verify that brackets, braces, and parentheses are balanced before attempting to run it on the Bracketeer™.

   **Instructions**

   Given a string containing brackets [], braces {}, parentheses (), or any combination thereof, verify that any and all pairs are matched and nested correctly. Any other characters should be ignored. For example, `"{what is (42)}?"` is balanced and `"[text}"` is not.

10. # `collatz()` Instruction

    The Collatz Conjecture or 3x+1 problem can be summarized as follows:

    Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is odd, multiply n by 3 and add 1 to get 3n + 1. Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.

    Given a number n, return the number of steps required to reach 1.

    Examples
    Starting with n = 12, the steps would be as follows:

    1. 12
    2. 6
    3. 3
    4. 10
    5. 5
    6. 16
    7. 8
    8. 4
    9. 2
    10. 1

    Resulting in 9 steps. So for input n = 12, the return value would be 9.

11. ## Diffie-Hellman key exchange Instruction.

    ### `private_key()`, `public_key()` & `secret()`

    Alice and Bob use Diffie-Hellman key exchange to share secrets. They start with prime numbers, pick private keys, generate and share public keys, and then generate a shared secret key.

    **Step 0**
    The test program supplies prime numbers p and g.

    **Step 1**
    Alice picks a private key, a, greater than 1 and less than p. Bob does the same to pick a private key b.

    **Step 2**
    Alice calculates a public key A.
    `A = gᵃ mod p` Using the same p and g, Bob similarly calculates a public key B from his private key b.

    **Step 3**
    Alice and Bob exchange public keys. Alice calculates secret key s.
    `s = Bᵃ mod p`

    Bob calculates
    `s = Aᵇ mod p`

    The calculations produce the same result! Alice and Bob now share secret s.

12. ## `series()` Instruction

    Given a string of digits, output all the contiguous substrings of length n in that string in the order that they appear.

    For example, the string "49142" has the following 3-digit series:

    - "491"
    - "914"
    - "142"

    And the following 4-digit series:

    - "4914"
    - "9142"
      And if you ask for a 6-digit series from a 5-digit string, you deserve whatever you get.

    > Note: that these series are only required to occupy adjacent positions in the input; the digits need not be numerically consecutive.

    Different languages on Exercism have different expectations about what the result should be if the length of the substrings is zero. On the Rust track, we don't have a test for that case, so you are free to do what you feel is most appropriate.

    Consider the advantages and disadvantages of the following possibilities:

    - Crash the program with panic!.
    - Return a Result::Err. (not possible here, because the function signature is given)
    - Return an empty vector.
    - Return a vector containing as many empty strings as the length of the string "digits" plus one. (this has some nice mathematical properties!)

13. ## Kindergarten Garden `plants()`
 Instruction 
    Your task is to, given a diagram, determine which plants each child in the kindergarten class is responsible for.

    There are 12 children in the class:

    Alice, Bob, Charlie, David, Eve, Fred, Ginny, Harriet, Ileana, Joseph, Kincaid, and Larry.
    Four different types of seeds are planted:

    | Plant  | Diagram encoding |
    | ------ | ---------------- |
    | Grass  | G                |
    | Clover | C                |
    | Radish | R                |
    | Violet | V                |

    Each child gets four cups, two on each row:

    [window][window][window]

    ........................ # each dot represents a cup

    ........................

    Their teacher assigns cups to the children alphabetically by their names, which means that Alice comes first and Larry comes last.

    Here is an example diagram representing Alice's plants:

    [window][window][window]

    VR......................

    RG......................

    In the first row, nearest the windows, she has a violet and a radish. In the second row she has a radish and some grass.

    Your program will be given the plants from left-to-right starting with the row nearest the windows. From this, it should be able to determine which plants belong to each student.

    For example, if it's told that the garden looks like so:

    [window][window][window]

    VRCGVVRVCGGCCGVRGCVCGCGV

    VRCCCGCRRGVCGCRVVCVGCGCV

    Then if asked for Alice's plants, it should provide:

    Violets, radishes, violets, radishes
    While asking for Bob's plants would yield:

    Clover, grass, clover, clover

14) ## Eliud's Eggs `egg_count()` Instruction 
    Your friend Eliud inherited a farm from her grandma Tigist. Her granny was an inventor and had a tendency to build things in an overly complicated manner. The chicken coop has a digital display showing an encoded number representing the positions of all eggs that could be picked up.

    Eliud is asking you to write a program that shows the actual number of eggs in the coop.

    The position information encoding is calculated as follows:

    Scan the potential egg-laying spots and mark down a 1 for an existing egg or a 0 for an empty spot.
    Convert the number from binary to decimal.
    Show the result on the display.

    Example

    Chicken Coop:

    ---

    |E| |E|E| | |E|

    Resulting Binary:

    1 0 1 1 0 0 1

    Decimal number on the display:

    89

    Actual eggs in the coop:

    4

    Example 2:

    Chicken Coop:

    ---

    | | | |E| | | | |

    Resulting Binary:

    0 0 0 1 0 0 0 0

    Decimal number on the display:

    16

    Actual eggs in the coop:

    1

    Your task is to count the number of 1 bits in the binary representation of a number.

15) ## `High_Score` Instruction

    Manage a game player's High Score list.

    Your task is to build a high-score component of the classic Frogger game, one of the highest selling and most addictive games of all time, and a classic of the arcade era. Your task is to write methods that return the highest score from the list, the last added score and the three highest scores.

16) ##  Luhn ckecsum `is_valid()` Instruction

    Given a number determine whether or not it is valid per the Luhn formula.

    The Luhn algorithm is a simple checksum formula used to validate a variety of identification numbers, such as credit card numbers and Canadian Social Insurance Numbers.

    The task is to check if a given string is valid.

    Validating a Number

    Strings of length 1 or less are not valid. Spaces are allowed in the input, but they should be stripped before checking. All other non-digit characters are disallowed.

    Example 1: valid credit card number

    4539 3195 0343 6467

    The first step of the Luhn algorithm is to double every second digit, starting from the right. We will be doubling

    4539 3195 0343 6467
    ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ (double these)

    If doubling the number results in a number greater than 9 then subtract 9 from the product. The results of our doubling:
    8569 6195 0383 3437
    Then sum all of the digits:

    8+5+6+9+6+1+9+5+0+3+8+3+3+4+3+7 = 80

    If the sum is evenly divisible by 10, then the number is valid. This number is valid!

    Example 2: invalid credit card number

    8273 1232 7352 0569

    Double the second digits, starting from the right

    7253 2262 5312 0539
    Sum the digits

    7+2+5+3+2+2+6+2+5+3+1+2+0+5+3+9 = 57

    57 is not evenly divisible by 10, so this number is not valid.

17) ## Acronym  `abbreviate()` Instruction

    Convert a phrase to its acronym.

    Techies love their TLA (Three Letter Acronyms)!

    Help generate some jargon by writing a program that converts a long name like Portable Network Graphics to its acronym (PNG).

    Punctuation is handled as follows: hyphens are word separators (like whitespace); all other punctuation can be removed from the input.

    For example:

    | Input                    | Output |
    | ------------------------ | ------ | ---- |
    | As Soon As Possible      | ASAP   |
    | Liquid-crystal display   | LCD    |
    | Thank George It's Friday | !      | TGIF |

18) ## All Your Base  `convert()` Instruction

    You've just been hired as professor of mathematics. Your first week went well, but something is off in your second week. The problem is that every answer given by your students is wrong! Luckily, your math skills have allowed you to identify the problem: the student answers are correct, but they're all in base 2 (binary)! Amazingly, it turns out that each week, the students use a different base. To help you quickly verify the student answers, you'll be building a tool to translate between bases.

    Convert a sequence of digits in one base, representing a number, into a sequence of digits in another base, representing the same number.

    The number 42, in base 10, means:

    `(4 × 10¹) + (2 × 10⁰)`

    The number 101010, in base 2, means:

    `(1 × 2⁵) + (0 × 2⁴) + (1 × 2³) + (0 × 2²) + (1 × 2¹) + (0 × 2⁰)`

    The number 1120, in base 3, means:

    `(1 × 3³) + (1 × 3²) + (2 × 3¹) + (0 × 3⁰)`

    Yes. Those three numbers above are exactly the same. Congratulations!

19) ## `allergies()` Instruction

      Given a person's allergy score, determine whether or not they're allergic to a given item, and their full list of allergies.

      An allergy test produces a single numeric score which contains the information about all the allergies the person has (that they were tested for).

      The list of items (and their value) that were tested are:

      eggs (1)

      peanuts (2)

      shellfish (4)

      strawberries (8)

      tomatoes (16)

      chocolate (32)

      pollen (64)

      cats (128)

      So if Tom is allergic to peanuts and chocolate, he gets a score of 34.

      Now, given just that score of 34, your program should be able to say:

      Whether Tom is allergic to any one of those allergens listed above.

      All the allergens Tom is allergic to.

      >Note: a given score may include allergens not listed above (i.e. allergens that score 256, 512, 1024, etc.). Your program should ignore those components of the score. For example, if the allergy score is 257, your program should only report the eggs (1) allergy.

20) ## Binary Search `find()` Instruction

      You have stumbled upon a group of mathematicians who are also singer-songwriters. They have written a song for each of their favorite numbers, and, as you can imagine, they have a lot of favorite numbers (like 0 or 73 or 6174).

      You are curious to hear the song for your favorite number, but with so many songs to wade through, finding the right song could take a while. Fortunately, they have organized their songs in a playlist sorted by the title — which is simply the number that the song is about.

      You realize that you can use a binary search algorithm to quickly find a song given the title.

      Instructions
      
      Your task is to implement a binary search algorithm.

      A binary search algorithm finds an item in a list by repeatedly splitting it in half, only keeping the half which contains the item we're looking for. It allows us to quickly narrow down the possible locations of our item until we find it, or until we've eliminated all possible locations.

      Caution

      Binary search only works when a list has been sorted.

      The algorithm looks like this:

      Find the middle element of a sorted list and compare it with the item we're looking for.
      If the middle element is our item, then we're done!

      If the middle element is greater than our item, we can eliminate that element and all the elements after it.

      If the middle element is less than our item, we can eliminate that element and all the elements before it.

      If every element of the list has been eliminated then the item is not in the list.
      Otherwise, repeat the process on the part of the list that has not been eliminated.
      Here's an example:

      Let's say we're looking for the number 23 in the following sorted list: [4, 8, 12, 16, 23, 28, 32].

      We start by comparing 23 with the middle element, 16.

      Since 23 is greater than 16, we can eliminate the left half of the list, leaving us with [23, 28, 32].

      We then compare 23 with the new middle element, 28.

      Since 23 is less than 28, we can eliminate the right half of the list: [23].
      We've found our item.