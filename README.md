# Advent of Code 2023

As usual, my goal is to run all days in under 1 second total run-time. I deliberately do not include file loads from disk since that varies wildly from machine to machine.


## Benchmarks

Only days with both solutions are shown.

```
🎄Day 1 (535.2µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 55488 (209.7µs)
Part 2: 55614 (325.5µs)

🎄Day 2 (119.7µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (113.4µs)
Part 1: 2685 (4.6µs)
Part 2: 83707 (1.7µs)

🎄Day 3 (1.9948ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.4749ms)
Part 1: 557705 (388.3µs)
Part 2: 84266818 (131.6µs)

🎄Day 4 (337.4µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (275.6µs)
Part 1: 20117 (37.3µs)
Part 2: 13768818 (24.5µs)

🎄Day 5 (88.2µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (32.7µs)
Part 1: 282277027 (28µs)
Part 2: 11554135 (27.5µs)

🎄Day 6 (8.4µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.1µs)
Part 1: 1731600 (5µs)
Part 2: 40087680 (2.3µs)

🎄Day 7 (1.0452ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (373µs)
Part 1: 249748283 (63.5µs)
Part 2: 248029057 (608.7µs)

🎄Day 8 (2.6747ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (347.4µs)
Part 1: 16271 (907.7µs)
Part 2: 14265111103729 (1.4196ms)

Final Runtime: 6.8036ms
```

## Lessons Learned

I always learn something each year.

> **Warning** There are spoilers below!

### Day 1

My initial plan was to replace the text with a regex pattern such as `r'(\d|one|two|three|four|five|six|seven|eight|nine)` and then do a pattern match to decode the digits. However, the puzzle is tricky and has strings like `oneight` and `eightwo`. The  [captures_iter](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures_iter) function _yields successive non-overlapping matches_. Unfortunately this means that the first digit would be matched, but not the second since the last letter was already consumed by the pattern, creating potentially incomplete parses. In other regex engines we can use a positive look-ahead like `r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))`. 

Unfortunately Rust's regex engine does not have support. Instead, I opted for some string replacement, which is probably faster anyway. The caveat being that we have to include the first and last letter within the replaced value, such as `nine` -> `n9e` so that the letters for other following numbers aren't dropped by accident. Actually rather interesting for a day 1 puzzle!

### Day 5

The trick here was to work on ranges, not on seeds themselves to greatly reduce the calculation space. Part 1 works on seeds directly and part 2 works on ranges to show the difference.