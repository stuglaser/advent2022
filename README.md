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
Day  1 |      91 µs  (21890 samples)
Day  2 |      91 µs  (21977 samples)
Day  3 |      79 µs  (25018 samples)
Day  4 |      63 µs  (31287 samples)
Day  5 |      43 µs  (46154 samples)
Day  6 |       9 µs  (216505 samples)
Theoretical total: 0.378814 ms
```

The rules I follow are roughly:
* Parsing the input counts as time spent (but it's ok to warm the disk cache).
* The solution should be general (try not to hack for a certain input).
* You can generally guess input sizes, but don't measure directly on the test input (for pre-reserving arrays, for example).
