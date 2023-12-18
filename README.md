# Advent of Code 2023

As usual, my goal is to run all days in under 1 second total run-time. I deliberately do not include file loads from disk since that varies wildly from machine to machine.


## Benchmarks

Only days with both solutions are shown.

```
🎄Day 1 (584.2µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 55488 (278.1µs)
Part 2: 55614 (306.1µs)

🎄Day 2 (109.2µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (104.6µs)
Part 1: 2685 (2.7µs)
Part 2: 83707 (1.9µs)

🎄Day 3 (1.8633ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.3613ms)
Part 1: 557705 (374.3µs)
Part 2: 84266818 (127.7µs)

🎄Day 4 (332.4µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (265µs)
Part 1: 20117 (42µs)
Part 2: 13768818 (25.4µs)

🎄Day 5 (86.9µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (27.7µs)
Part 1: 282277027 (28.3µs)
Part 2: 11554135 (30.9µs)

🎄Day 6 (13µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.1µs)
Part 1: 1731600 (10.2µs)
Part 2: 40087680 (1.7µs)

🎄Day 7 (1.0597ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (356.1µs)
Part 1: 249748283 (73.9µs)
Part 2: 248029057 (629.7µs)

🎄Day 8 (2.8489ms) 🎄
Parse : (341.9µs)
Part 1: 16271 (913.1µs)
Part 2: 14265111103729 (1.5939ms)

🎄Day 9 (639.1µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 1938731307 (315.9µs)
Part 2: 948 (323.2µs)

🎄Day 11 (10.7496ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 9724940 (5.3787ms)
Part 2: 569052586852 (5.3709ms)

🎄Day 14 (2.5493829s) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (121.8µs)
Part 1: 106990 (2.9407ms)
Part 2: 100531 (2.5463204s)

🎄Day 15 (532.2µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 494980 (82.8µs)
Part 2: 247933 (449.4µs)

Final Runtime: 2.5682014s
```

## Lessons Learned

I always learn something each year.

> **Warning** There are spoilers below!

### Day 1

My initial plan was to replace the text with a regex pattern such as `r'(\d|one|two|three|four|five|six|seven|eight|nine)` and then do a pattern match to decode the digits. However, the puzzle is tricky and has strings like `oneight` and `eightwo`. The  [captures_iter](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures_iter) function _yields successive non-overlapping matches_. Unfortunately this means that the first digit would be matched, but not the second since the last letter was already consumed by the pattern, creating potentially incomplete parses. In other regex engines we can use a positive look-ahead like `r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))`. 

Unfortunately Rust's regex engine does not have support. Instead, I opted for some string replacement, which is probably faster anyway. The caveat being that we have to include the first and last letter within the replaced value, such as `nine` -> `n9e` so that the letters for other following numbers aren't dropped by accident. Actually rather interesting for a day 1 puzzle!

### Day 5

The trick here was to work on ranges, not on seeds themselves to greatly reduce the calculation space. Part 1 works on seeds directly and part 2 works on ranges to show the difference.