# Advent of Code 2023

As usual, my goal is to run all days in under 1 second total run-time. I deliberately do not include file loads from disk since that varies wildly from machine to machine.


## Benchmarks

Only days with both solutions are shown.

```
🎄Day 1 (641.4µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 55488 (246.4µs)
Part 2: 55614 (395µs)

🎄Day 2 (114.5µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (110µs)
Part 1: 2685 (2.8µs)
Part 2: 83707 (1.7µs)

🎄Day 3 (1.8477ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.3472ms)
Part 1: 557705 (377.2µs)
Part 2: 84266818 (123.3µs)

🎄Day 4 (354.3µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (287.1µs)
Part 1: 20117 (41µs)
Part 2: 13768818 (26.2µs)

🎄Day 5 (98.3µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (35.9µs)
Part 1: 282277027 (36.1µs)
Part 2: 11554135 (26.3µs)

🎄Day 6 (9.4µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.1µs)
Part 1: 1731600 (5.9µs)
Part 2: 40087680 (2.4µs)

🎄Day 7 (1.0876ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (378.3µs)
Part 1: 249748283 (74.7µs)
Part 2: 248029057 (634.6µs)

🎄Day 8 (3.5191ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (346.7µs)
Part 1: 16271 (927.2µs)
Part 2: 14265111103729 (2.2452ms)

🎄Day 9 (584.7µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 1938731307 (319.4µs)
Part 2: 948 (265.3µs)

🎄Day 11 (10.875ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 9724940 (5.4424ms)
Part 2: 569052586852 (5.4326ms)

🎄Day 15 (552.1µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 494980 (89.6µs)
Part 2: 247933 (462.5µs)

Final Runtime: 19.6841ms
```

## Lessons Learned

I always learn something each year.

> **Warning** There are spoilers below!

### Day 1

My initial plan was to replace the text with a regex pattern such as `r'(\d|one|two|three|four|five|six|seven|eight|nine)` and then do a pattern match to decode the digits. However, the puzzle is tricky and has strings like `oneight` and `eightwo`. The  [captures_iter](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures_iter) function _yields successive non-overlapping matches_. Unfortunately this means that the first digit would be matched, but not the second since the last letter was already consumed by the pattern, creating potentially incomplete parses. In other regex engines we can use a positive look-ahead like `r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))`. 

Unfortunately Rust's regex engine does not have support. Instead, I opted for some string replacement, which is probably faster anyway. The caveat being that we have to include the first and last letter within the replaced value, such as `nine` -> `n9e` so that the letters for other following numbers aren't dropped by accident. Actually rather interesting for a day 1 puzzle!

### Day 5

The trick here was to work on ranges, not on seeds themselves to greatly reduce the calculation space. Part 1 works on seeds directly and part 2 works on ranges to show the difference.