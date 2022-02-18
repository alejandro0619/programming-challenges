1) Calculate how many months have passed from the beginning of the year until a given month. </br>
[Rust solution](https://github.com/alejandro0619/programming-challenges/tree/main/exercises/calculate-month-until-date-rs) </br>
Other solution: </br>
[Typescript solution](https://github.com/alejandro0619/programming-challenges/tree/main/exercises/calculate_month_until_date_TS/) </br>

2) Given an integer, loop through it, that means, in a range: (0..integers) and for each loop, display those numbers that are odds. </br>
[Rust solution](https://github.com/alejandro0619/programming-challenges/tree/main/exercises/loop-integers-rs/) </br>
[Typescript solution](https://github.com/alejandro0619/programming-challenges/tree/main/loop-integers-ts) </br>

3) Given a word, return the distance between the nearest vowel to the beginning of that word. </br>
[Rust solution](https://github.com/alejandro0619/programming-challenges/tree/main/exercises/nearest-vowel-rs) </br>
[Rust solution](https://github.com/alejandro0619/programming-challenges/tree/main/exercises/nearest-vowel-ts) </br>
4) Have the function ```FirstFactorial(num)``` take the num parameter being passed and return the factorial of it. For example: ```if num = 4```, then your program should return ```(4 * 3 * 2 * 1) = 24```. For the test cases, the range will be between 1 and 18 and the input will always be an integer. </br>
[Rust solution](https://github.com/alejandro0619/programming-challenges/tree/main/first-factorial-rs) </br>
[Typescript solution](https://github.com/alejandro0619/programming-challenges/tree/main/first-factorial-ts)

5) Complete the solution so that it returns true if the first argument(string) passed in ends with the 2nd argument (also a string). Examples: </br>
[Rust Solution](https://github.com/alejandro0619/programming-challenges/tree/main/string-ending-rs) </br>
[Typescript Solution](https://github.com/alejandro0619/programming-challenges/tree/main/string-ending-ts) 
```solution('abc', 'bc') 
// returns true solution('abc', 'd') // returns false
```

6) Given the hours and cost per hour, calculate the payment to a worker. </br>
[Rust Solution](https://github.com/alejandro0619/programming-challenges/tree/main/worker-salary-rs) </br>
[Typescript Solution](https://github.com/alejandro0619/programming-challenges/tree/main/worker-salary-ts)

7) Good vs Evil - This challengue has its own readme inside its repository to avoid flood this readme's section.
[Rust Solution](https://github.com/alejandro0619/programming-challenges/tree/main/good-vs-evil-rs) </br>
[Typescript Solution](https://github.com/alejandro0619/programming-challenges/tree/main/good-vs-evil-ts) </br>

8) Write a simple parser that will parse and run Deadfish. </br>
Deadfish has 4 commands, each 1 character long: </br>
**i** increments the value (initially 0) </br>
**d** decrements the value </br>
**s** squares the value </br>
**o** outputs the value into the return array </br>
Invalid characters should be ignored.</br>
[Rust Solution](https://github.com/alejandro0619/programming-challenges/tree/main/deadfish-swim-rs) </br>
``` 
parse("iiisdoso") => [ 8, 64 ] 
```

9) Write a function that takes in a string of one or more words, and returns the same string, but with all five or more letter words reversed (Just like the name of this Kata). Strings passed in will consist of only letters and spaces. Spaces will be included only when more than one word is present.Examples: 
</br>
[Rust Solution](https://github.com/alejandro0619/programming-challenges/tree/main/spin-words-rs) </br>

``` 
spinWords( "This is a test") => returns "This is a test" 
spinWords( "This is another test" )=> returns "This is rehtona test"
 ```
10) In this kata you are required to, given a string, replace every letter with its position in the alphabet. If anything in the text isn't a letter, ignore it and don't return it: ```"a" = 1, "b" = 2, etc...```
</br>
[Rust Solution](https://github.com/alejandro0619/programming-challenges/tree/main/alphabet-position-rs)

```
alphabet_position("The sunset sets at twelve o' clock.")
Should return 
"20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11" 
```
11) If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
</br>
Finish the solution so that it returns the sum of all the multiples of 3 or 5 below the number passed in. Additionally, if the number is negative, return 0 (for languages that do have them).
</br>
Note: If the number is a multiple of both 3 and 5, only count it once.
[Rust Solution](https://github.com/alejandro0619/programming-challenges/tree/main/multiples-of-3-and-5-rs)