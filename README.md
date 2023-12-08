# Advent of Code 2023

As usual, my goal is to run all days in under 1 second total run-time. I deliberately do not include file loads from disk since that varies wildly from machine to machine.


## Benchmarks

Only days with both solutions are shown.

```
🎄Day 1 (566.8µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Part 1: 55488 (206µs)
Part 2: 55614 (360.8µs)

🎄Day 2 (107.2µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (102.2µs)
Part 1: 2685 (2.9µs)
Part 2: 83707 (2.1µs)

🎄Day 3 (1.9143ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.4056ms)
Part 1: 557705 (382.5µs)
Part 2: 84266818 (126.2µs)

🎄Day 4 (331.3µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (269.3µs)
Part 1: 20117 (37.6µs)
Part 2: 13768818 (24.4µs)

🎄Day 5 (95µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (30.3µs)
Part 1: 282277027 (34.5µs)
Part 2: 11554135 (30.2µs)

🎄Day 6 (9.2µs) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (1.4µs)
Part 1: 1731600 (5.6µs)
Part 2: 40087680 (2.2µs)

🎄Day 7 (1.047ms) 🎄
~~~~~~~~~~~~~~~~~~~~~
Parse : (364.5µs)
Part 1: 249748283 (75.3µs)
Part 2: 248029057 (607.2µs)

Final Runtime: 4.0708ms
```

## Lessons Learned

I always learn something each year.

> **Warning** There are spoilers below!

### Day 1

My initial plan was to replace the text with a regex pattern such as `r'(\d|one|two|three|four|five|six|seven|eight|nine)` and then do a pattern match to decode the digits. However, the puzzle is tricky and has strings like `oneight` and `eightwo`. The  [captures_iter](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures_iter) function _yields successive non-overlapping matches_. Unfortunately this means that the first digit would be matched, but not the second since the last letter was already consumed by the pattern, creating potentially incomplete parses. In other regex engines we can use a positive look-ahead like `r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))`. 

Unfortunately Rust's regex engine does not have support. Instead, I opted for some string replacement, which is probably faster anyway. The caveat being that we have to include the first and last letter within the replaced value, such as `nine` -> `n9e` so that the letters for other following numbers aren't dropped by accident. Actually rather interesting for a day 1 puzzle!

### Day 5

The trick here was to work on ranges, not on seeds themselves to greatly reduce the calculation space. Part 1 works on seeds directly and part 2 works on ranges to show the difference.