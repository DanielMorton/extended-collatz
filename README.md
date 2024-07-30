# The Collatz Conjecture
The Collatz Conjecture is a famous, and deceptively simple, unsolved problem in mathematics. The algorithm it concerns is simplicity itself; if a number is odd, multiply by 3 and add 1, 
otherwise divide by 2. The conjecture states that for any starting value the algorithm will eventually reach 1. The problem has remained unsolved since 1937.

There is a heuristic argument that suggests why the conjecture should be true. The Collatz algorithm can be reformulated as follows. Start with an odd number. Multiply by 3 and add 1. Then divide by
two until the number is odd again. (In binary notation this ammounts to shifting digits to the right until the last digit is 1.) Repeat. From this perspective it is obvious that it is sufficent to only consider
odd starting numbers since even starting values will always be divided until they are odd. After applying the 3n+1 step the output will always be even. There is a 50% chance that the output will be
divisible by 2 but not by 4, a 25% chance that the output will be divisible by 4 but not by 8, a 12.5% chance that the output will be divisible by 8 but not 16. The expected value of the output after one
iteration of the algorithm is (3n + 1)/4, which is always less than n if n is greater than 1. For any starting value, the sequence of values *should* assymtotically decrease.

The conjecture has been confirmed up to 10<sup>22</sup>.

## A Generalized Collatz Conjecture

An obvious way to generalize the Collatz Conjecture is to replace the number 3 with some other number. Done naively, this will fail badly. Replacing 3 with 5 and using the same argument as above yields
a sequence whose values should assymptotically increase by a factor of 5/4 at each step. Starting with a value of n = 9 and applying 5n + 1 to odd n and n/2 to even n yields a sequence that goes to
infinity. A slight change produces series that are better behaved. Instead of applyng 5n + 1 when n is odd, apply 5n + 1 or 5n + 3 depending on whether 5n is 1 or 3 less than a multiple of 4. The resulting
sseries should now, on average, decrease bby a factor of 5/8 at each step.

It turns out that this algorithm does not alway reach 1, but it appears to reach one of four loops: [1], [31, 39, 49], [37, 47, 59], [61, 77, 97]. This has been tested up to 10<sup>9<sup>. The relative
proportions of the four loops appear to be releatively constant as well (63%, 13%, 20%, and 4% in round numbers).

The general conjecture then takes the following form. Take a number p and an odd number a where 2<sup>p</sup> < a < 2<sup>p+1</sup>. Then the algorithm defined by 

n := a * n

n := n + (2<sup>p</sup> - n % 2<sup>p</sup>) (To ensure the output is divisible by 2<sup>p</sup>.)

when n is odd and

n := n/2

when n is even will eventually reach a finite loop. Furthermore, for each value of a, there are finite number of possible loops.
