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
Day  1 |      90 µs  (22003 samples)
Day  2 |      93 µs  (21425 samples)
Day  3 |      77 µs  (25955 samples)
Day  4 |      69 µs  (28806 samples)
Day  5 |      46 µs  (43114 samples)
Day  6 |       9 µs  (218617 samples)
Day  7 |     211 µs  (9443 samples)
Day  8 |     526 µs  (3802 samples)
Day  9 |     552 µs  (3618 samples)
Day 10 |       6 µs  (291643 samples)
Day 11 |   4,358 µs  (459 samples)
Day 12 |      22 µs  (88446 samples)
Day 13 |     410 µs  (4869 samples)
Day 14 |   3,402 µs  (588 samples)
Day 15 |  42,485 µs  (48 samples)
Day 16 | 1,351,391 µs  (2 samples)
Day 17 |     526 µs  (3798 samples)
Day 18 |     673 µs  (2972 samples)
Theoretical total: 1404.955045 ms

```

The rules I follow are roughly:
* Parsing the input counts as time spent (but it's ok to warm the disk cache).
* The solution should be general (try not to hack for a certain input).
* You can generally guess input sizes, but don't measure directly on the test input (for pre-reserving arrays, for example).
