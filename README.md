# Advent of Code 2023

As usual, my goal is to run all days in under 1 second total run-time. I deliberately do not include file loads from disk since that varies wildly from machine to machine.


## Benchmarks

Only days with both solutions are shown.

| Day       | Result  | 
|-----------|---------|
| 01        | 561.1µs |
| 02        | 195.5µs |
| **Total** | 756.6µs |

## Lessons Learned

I always learn something each year.

> **Warning** There are spoilers below!

### Day 1

My initial plan was to replace the text with a regex pattern such as `r'(\d|one|two|three|four|five|six|seven|eight|nine)` and then do a pattern match to decode the digits. However, the puzzle is tricky and has strings like `oneight` and `eightwo`. The  [captures_iter](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures_iter) function _yields successive non-overlapping matches_. Unfortunately this means that the first digit would be matched, but not the second since the last letter was already consumed by the pattern, creating potentially incomplete parses. In other regex engines we can use a positive look-ahead like `r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))`. 

Unfortunately Rust's regex engine does not have support. Instead, I opted for some string replacement, which is probably faster anyway. The caveat being that we have to include the first and last letter within the replaced value, such as `nine` -> `n9e` so that the letters for other following numbers aren't dropped by accident. Actually rather interesting for a day 1 puzzle!
