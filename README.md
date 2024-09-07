# The Collatz Conjecture
The Collatz Conjecture is a famous, and deceptively simple, unsolved problem in mathematics. The algorithm it concerns is simplicity itself; if a number is odd, multiply by 3 and add 1, 
otherwise divide by 2. The conjecture states that for any starting value the algorithm will eventually reach 1. The problem has remained unsolved since 1937.

There is a heuristic argument that suggests why the conjecture should be true. The Collatz algorithm can be reformulated as follows. Start with an odd number. Multiply by 3 and add 1. Then divide by
two until the number is odd again.  Repeat. From this perspective it is obvious that it is sufficient to only consider
odd starting numbers since even starting values will always be divided until they are odd. After applying the 3n+1 step the output will always be even. Thus starting with a value n, half the time
the next odd number will be (3n + 1)/2, half of the remaining time (25%) it will be (3n + 1)/4, and half of the times left (12.5%) it will
be (3n + 1)/8. Taking the expected value we see that, on average, the next odd number should be 3/4 of it's predecessor, and
thus the sequence *should* at least converge to a cycle. In light of this, and the numerical evidence, we can expect
1 -> 4 -> 2 -> 1 (or 1 -> 1 excluding even numbers) to be the only cycle.

The conjecture has been confirmed up to 10<sup>22</sup>.

## Extending the  Collatz Conjecture

In order to get convergence to a cycle, we need to preserve the heuristic argument above. A naive generalization would
simply replace 3n + 1 with 5n + 1. This shouldn't work. The heuristic argument suggests that, on average, each odd number
will be 5/4 times it's predecessor. Starting with n = 7 produces a sequence that at least appears to go
to infiniity. (Even this has not been proven; nothing related to the Collatz Conjecture is easy.) If we could replace the heuristic
factor of 5/4 with something less than one, such as 5/8, we should expect convergence. This can be achieved by ensuring that
each odd number is followed by a multiple of 4. In cases where 5n + 1 is only divisible by 2 we replace it with 5n + 3.

The new algorithm is then as follows. Start with an odd number n. Multiply by 5. Add 1 or 3 to ensure that the result is a multiple of 4.
Then divide by 2 until the result is odd. Repeat.

The natural generalization of the Collatz Conjecture would be that this sequence is eventually one. It only takes n = 23 to show that this
is false. Starting with n = 23 (and excluding even numbers,) we get the sequence 23 -> 29 -> 37 -> 47 -> 59 -> 37. This shows that some sequences reach a non-trivial
cycle, namely 37 -> 47 -> 59 -> 37. Curiously, there appear to be only two other such cycles for this algorithm: 31 -> 39 -> 49
and 61 -> 77 -> 97. Even more curiously, these four cycles (including 1 -> 1) occur with relatively constant frequencies.

| Cycle  | Frequency |
|:-------|----------:|
| 1 -> 1 |     62.7% |
| 31 -> 39 -> 49 |     13.6% |
| 37 -> 47 -> 59 | 19.7% |
| 61 -> 77 -> 97 | 4.0 % |

These percentages, or something very close to then, hold true for all sequences starting with odd numbers less than 10000,
all odd numbers less than 1 million, or simply all odd numbers in any (sufficiently large) interval.

I have verified this for all sequences starting with odd numbers less than 1 billion. Most importantly,
every sequence eventually reaches some finite cycle. As predicted by the heuristic, no sequence has gone
to infinity. As noted before the frequency of the four cycles remains consistent at any scale. It is also worth
noting that the non-trivial cycles all have the same length. The fact that the non-trivial cycles all start with
prime numbers is coincidence.

### 7n + 1

It's trivial to replace 5 with 7. Starting with an odd number, multiply by 7 and add 1 or 3, whichever yields a multiple of 4. Divide
by two until the number is odd again. Once
again, the heuristic argument says the sequence should, on average, decrease, this time by a factor of 7/8. The same numerical
analysis shows that every sequence reaches one of two cycles, both trivial if even numbers are ignored. They are 1 -> 1 and 3 -> 3. Once
again these cycles occur at a fixed ratio, and all this holds true for any odd number less than 1 billion.

| Cycle | Frequency |
|:-------|----------:|
| 1 -> 1 |     86.2% |
| 3 -> 3|     13.8% |

## A Generalized Collatz Algorithm

It should now be obvious that if 7 is replaced with the 9 but the +1 or +3 rule is retained then the
heuristic argument suggests that the resulting sequence would increase, on average, by a rate of 9/8 and
thus go off to infinity. The key fact is that by going from 7 to 9 we jumped over a power of 2.
When we do that we need to change the addition rule to ensure that the result is divisible by
a higher power of 2. Replacing the +1 or +3 rule with a +1, +3, +5, or +7 (whichever yields a 
multiple of 8) rule then any sequence should, on average, decrease by a factor of 9/16 and thus reach
a finite cycle. The same rule should work if we replace the multiplicative factor of 9 by 11, 13, or 15
but once we get to 17 the additive rule will have to get more complicated.

We now have a way to generalize the Collatz Algorithm. Any odd number a lies between two powers of 2, which we denote
p and 2p. Starting with any odd number n, we multiply by a and then add the smallest positive number
such that the result is divisible by p and then divide by 2 until the number is odd again. If we
repeat these steps the heuristic argument says that the next odd number should be, on average,
a/(2p) times it's predecessor. Since a/(2p) < 1, we should expect each sequence, regardless of starting
value, reach some finite cycle.

