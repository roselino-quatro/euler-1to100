# Project Euler 1 to 100

> *"Project Euler exists to encourage, challenge, and develop the skills and enjoyment of anyone with an interest in the fascinating world of mathematics."*

Project Euler is a series of mathematical problems which can *usually* only be reasonably solved by using programming. In this repository I will post my solutions of problems 1 to 100, specifically using the language Rust.

## Why are you even doing this?

Welp, I want to learn the syntax of Rust and I want to play a little bit with math, so I dont get *rusty* at it. Yeah I am extremely funny I know thank you a lot.

## Do you post answers?

Nop, but you can just run the code and get the answer, the Project Euler website says:

> *"the rule about sharing solutions outside of Project Euler does not apply to the first one-hundred problems, as long as any discussion clearly aims to instruct methods, not just provide answers, and does not directly threaten to undermine the enjoyment of solving later problems."*

so I will try to follow this rule, giving answers is no fun, but I think its fine to show some code and my thought process.

In addition to this every solution will have my first try at solving the problem, my last (and hopefully) best method and a little text explaining my ideas and proofs to the specific problem.

## List of solutions:

- 1 - Multiples of 3 and 5

    The intuitive method to solving this exercise is a brute force for loop, checking every single number from 1 to the target (in this casem 1000), the problem with this method is simples: it gets very inneficient very fast, going through so many numbers will eventually make this solution unreasonable for bigger and bigger numbers, so we need to find a better way

    The faster solution takes advantage from noticing that we actually are just summing up every number in a arithmetic progression: 3, 6, 9 .. 999 + 5, 10, 15 ... 995. Now we just need to use the arithmetic series with the right variables and find the solution!

- 2 - Even Fibonacci numbers

    The first reaction to this problem and simplest solution is making a fibonacci recursive function and check every number until it gets bigger than 4 million, this is good but if we just calculate the even numbers from fibonacci sequence we can not only have no need to check if the number is even or not but also make a lot less equations. Let's look at the numbers in fibonacci to see if we can find a pattern:

    1 1 *2* 3 5 *8* 13 21 *34* 55

    So we have 2, 8, 34... we can notice that 1- we get one even number every 2 odds number 2- every next even number is made from 4c + p where c is the current even number and p is the previous even number. (If you want to see a good proof for this checkout the Euler forum thread about this problem).

    With this information we can pretty easily make a simple function that calculates even fibonacci numbers! After that just put that on a loop for until it gets to 4 million.
