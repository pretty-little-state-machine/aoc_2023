# Advent of Code 2023

As usual, my goal is to run all days in under 1 second total run-time. I deliberately do not include file loads from disk since that varies wildly from machine to machine.


## Benchmarks

Only days with both solutions are shown.

```
🎄Day 1 (665.6µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 55488 (232µs)
Part 2: 55614 (433.6µs)

🎄Day 2 (115.4µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (110.2µs)
Part 1: 2685 (3.3µs)
Part 2: 83707 (1.9µs)

🎄Day 3 (1.9519ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.4346ms)
Part 1: 557705 (396.2µs)
Part 2: 84266818 (121.1µs)

🎄Day 4 (422.9µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (354.8µs)
Part 1: 20117 (42.4µs)
Part 2: 13768818 (25.7µs)

🎄Day 5 (98.8µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (35.4µs)
Part 1: 282277027 (37.2µs)
Part 2: 11554135 (26.2µs)

Final Runtime: 3.2546ms
```

## Lessons Learned

I always learn something each year.

> **Warning** There are spoilers below!

### Day 1

My initial plan was to replace the text with a regex pattern such as `r'(\d|one|two|three|four|five|six|seven|eight|nine)` and then do a pattern match to decode the digits. However, the puzzle is tricky and has strings like `oneight` and `eightwo`. The  [captures_iter](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures_iter) function _yields successive non-overlapping matches_. Unfortunately this means that the first digit would be matched, but not the second since the last letter was already consumed by the pattern, creating potentially incomplete parses. In other regex engines we can use a positive look-ahead like `r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))`. 

Unfortunately Rust's regex engine does not have support. Instead, I opted for some string replacement, which is probably faster anyway. The caveat being that we have to include the first and last letter within the replaced value, such as `nine` -> `n9e` so that the letters for other following numbers aren't dropped by accident. Actually rather interesting for a day 1 puzzle!

### Day 5

The trick here was to work on ranges, not on seeds themselves to greatly reduce the calculation space. Part 1 works on seeds directly and part 2 works on ranges to show the difference.