# Advent of Code 2023

As usual, my goal is to run all days in under 1 second total run-time. I deliberately do not include file loads from disk since that varies wildly from machine to machine.


## Benchmarks

Only days with both solutions are shown.

```
🎄Day 1 (633.541µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 55488 (283.125µs)
Part 2: 55614 (350.416µs)

🎄Day 2 (95.999µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (91.625µs)
Part 1: 2685 (2.708µs)
Part 2: 83707 (1.666µs)

🎄Day 3 (1.192334ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (702.417µs)
Part 1: 557705 (348.875µs)
Part 2: 84266818 (141.042µs)

🎄Day 4 (357.958µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (280.25µs)
Part 1: 20117 (39.208µs)
Part 2: 13768818 (38.5µs)

🎄Day 5 (153.125µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (31.75µs)
Part 1: 282277027 (95.625µs)
Part 2: 11554135 (25.75µs)

🎄Day 6 (2.791µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.166µs)
Part 1: 1731600 (250ns)
Part 2: 40087680 (1.375µs)

🎄Day 7 (853.875µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (304.5µs)
Part 1: 249748283 (96.375µs)
Part 2: 248029057 (453µs)

🎄Day 8 (1.876292ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (173.25µs)
Part 1: 16271 (607.5µs)
Part 2: 14265111103729 (1.095542ms)

🎄Day 9 (404.875µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 1938731307 (214.875µs)
Part 2: 948 (190µs)

🎄Day 11 (4.559292ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 9724940 (2.292917ms)
Part 2: 569052586852 (2.266375ms)

Final Runtime: 10.130082ms
```

## Lessons Learned

I always learn something each year.

> **Warning** There are spoilers below!

### Day 1

My initial plan was to replace the text with a regex pattern such as `r'(\d|one|two|three|four|five|six|seven|eight|nine)` and then do a pattern match to decode the digits. However, the puzzle is tricky and has strings like `oneight` and `eightwo`. The  [captures_iter](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures_iter) function _yields successive non-overlapping matches_. Unfortunately this means that the first digit would be matched, but not the second since the last letter was already consumed by the pattern, creating potentially incomplete parses. In other regex engines we can use a positive look-ahead like `r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))`. 

Unfortunately Rust's regex engine does not have support. Instead, I opted for some string replacement, which is probably faster anyway. The caveat being that we have to include the first and last letter within the replaced value, such as `nine` -> `n9e` so that the letters for other following numbers aren't dropped by accident. Actually rather interesting for a day 1 puzzle!

### Day 5

The trick here was to work on ranges, not on seeds themselves to greatly reduce the calculation space. Part 1 works on seeds directly and part 2 works on ranges to show the difference.