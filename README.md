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
Day  1 |      87 µs  (22751 samples)
Day  2 |      92 µs  (21689 samples)
Day  3 |      80 µs  (24865 samples)
Day  4 |      67 µs  (29707 samples)
Day  5 |      47 µs  (42091 samples)
Day  6 |       9 µs  (205727 samples)
Day  7 |     214 µs  (9324 samples)
Day  8 |     534 µs  (3740 samples)
Day  9 |     561 µs  (3562 samples)
Day 10 |       6 µs  (286882 samples)
Day 11 |   4,130 µs  (485 samples)
Day 12 |      23 µs  (86472 samples)
Theoretical total: 5.856534 ms
```

The rules I follow are roughly:
* Parsing the input counts as time spent (but it's ok to warm the disk cache).
* The solution should be general (try not to hack for a certain input).
* You can generally guess input sizes, but don't measure directly on the test input (for pre-reserving arrays, for example).
