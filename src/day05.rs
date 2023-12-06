use crate::DayResult;
use fxhash::FxHashMap;
use rayon::prelude::*;
use std::ops::Range;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let (seeds, mappers) = parse_input(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&seeds, &mappers).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&seeds, &mappers).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

#[derive(Debug)]
struct Mapper {
    source: String,
    destination: String,
    ranges: Vec<(Range<usize>, Range<usize>)>,
}

impl Mapper {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let mut header = lines.next().unwrap().split(' ');
        let mut header_fields = header.next().unwrap().split("-to-");
        let source = header_fields.next().unwrap().to_string();
        let destination = header_fields.next().unwrap().to_string();

        let mut ranges: Vec<(Range<usize>, Range<usize>)> = Vec::with_capacity(50);

        for line in lines {
            let mut fields = line.split(' ');
            let destination_range_start = fields.next().unwrap().parse::<usize>().unwrap();
            let source_range_start = fields.next().unwrap().parse::<usize>().unwrap();
            let range_length = fields.next().unwrap().parse::<usize>().unwrap();
            ranges.push((
                Range {
                    start: source_range_start,
                    end: source_range_start + range_length,
                },
                Range {
                    start: destination_range_start,
                    end: destination_range_start + range_length,
                },
            ));
        }

        Mapper {
            source,
            destination,
            ranges,
        }
    }

    fn get_target(&self, value: usize) -> usize {
        for (src_range, dst_range) in &self.ranges {
            if src_range.contains(&value) {
                return dst_range.start + (value - src_range.start);
            }
        }
        value
    }
}

fn parse_input(input: &str) -> (Vec<usize>, FxHashMap<String, Mapper>) {
    let mut mappers = FxHashMap::default();
    let mut seeds: Vec<usize> = Vec::with_capacity(30);
    let mut chunks = input.split("\n\n");

    let mut seed_values = chunks.next().unwrap().split(' ');
    seed_values.next(); // Skip the `seeds:` value
    for seed in seed_values {
        seeds.push(seed.parse::<usize>().unwrap());
    }

    for c in chunks {
        let mapper = Mapper::new(c);
        mappers.insert(mapper.source.clone(), mapper);
    }
    (seeds, mappers)
}

fn part_1(seeds: &[usize], mappers: &FxHashMap<String, Mapper>) -> usize {
    seeds
        .par_iter()
        .map(|seed| {
            let mut current_value = *seed;
            let mut current_target = "seed".to_string();

            while let Some(mapper) = mappers.get(&current_target) {
                current_value = mapper.get_target(current_value);
                current_target = mapper.destination.clone();
            }
            current_value
        })
        .min()
        .unwrap()
}

fn part_2(seeds: &[usize], mappers: &FxHashMap<String, Mapper>) -> usize {
    // Seed ranges are NOT inclusive!
    let mut seed_ranges = seeds
        .chunks(2)
        .map(|c| (c[0]..(c[0] + c[1] - 1)))
        .collect::<Vec<Range<usize>>>();

    let mut mapname = "seed".to_string();
    while let Some(mapper) = mappers.get(&mapname) {
        let mut ranges_to_append: Vec<Range<usize>> = Vec::new();

        'outer: for seed_range in &mut seed_ranges {
            for (src, dst) in &mapper.ranges {
                let offset = (dst.start as isize) - (src.start as isize);

                if seed_range.start >= src.start && seed_range.end < src.end {
                    // Full Overlap
                    seed_range.start = (seed_range.start as isize + offset) as usize;
                    seed_range.end = (seed_range.end as isize + offset) as usize;
                    continue 'outer;
                } else if seed_range.start < src.start && seed_range.end >= src.start {
                    // Partially Overlapping
                    if seed_range.end <= src.end {
                        ranges_to_append.push(Range {
                            start: (src.start as isize + offset) as usize,
                            end: ((seed_range.end as isize) + offset) as usize,
                        });
                        seed_range.end = src.start - 1;
                    } else {
                        ranges_to_append.push(Range {
                            start: (src.start as isize + offset) as usize,
                            end: ((src.end as isize) + offset) as usize,
                        });
                        ranges_to_append.push(Range {
                            start: src.end + 1,
                            end: seed_range.end,
                        });
                        seed_range.end = src.start - 1;
                    }
                } else if seed_range.start >= src.start && seed_range.start <= src.end {
                    ranges_to_append.push(Range {
                        start: ((seed_range.start as isize) + offset) as usize,
                        end: ((src.end as isize) + offset) as usize - 1,
                    });
                    seed_range.start = src.end;
                }
            }
        }
        seed_ranges.append(&mut ranges_to_append);
        mapname = mapper.destination.clone();
    }
    seed_ranges.iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let (seeds, mappers) = parse_input(input);
        assert_eq!(part_1(&seeds, &mappers), 35);
    }

    #[test]
    fn test_part_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let (seeds, mappers) = parse_input(input);
        assert_eq!(part_2(&seeds, &mappers), 46);
    }
}
