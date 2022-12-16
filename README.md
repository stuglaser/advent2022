This is an attempt to finish all problems in Advent of Code 2022 in under 1 second, similar
to my 2021 solutions.

Inspired by:
* https://www.forrestthewoods.com/blog/solving-advent-of-code-in-under-a-second/
* https://old.reddit.com/r/adventofcode/comments/kkq6r3/2020_optimized_solutions_in_c_291_ms_total/


Total runtime (all days):
```
All days
Took ...
```

```
Day  1 |      88 µs  (22694 samples)
Day  2 |      92 µs  (21672 samples)
Day  3 |      74 µs  (26938 samples)
Day  4 |      61 µs  (32531 samples)
Day  5 |      46 µs  (43035 samples)
Day  6 |       9 µs  (215084 samples)
Day  7 |     219 µs  (9118 samples)
Day  8 |     531 µs  (3764 samples)
Day  9 |     551 µs  (3626 samples)
Day 10 |       6 µs  (292927 samples)
Day 11 |   4,370 µs  (458 samples)
Day 12 |      22 µs  (88346 samples)
Day 13 |     398 µs  (5022 samples)
Day 14 |   3,394 µs  (590 samples)
Day 15 |  40,619 µs  (50 samples)
Theoretical total: 50.486509 ms
```

The rules I follow are roughly:
* Parsing the input counts as time spent (but it's ok to warm the disk cache).
* The solution should be general (try not to hack for a certain input).
* You can generally guess input sizes, but don't measure directly on the test input (for pre-reserving arrays, for example).
