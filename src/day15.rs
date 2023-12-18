use crate::DayResult;
use std::collections::LinkedList;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let p1 = part_1(input).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(input).to_string();
    let p2_duration = start.elapsed();
    (None, (p1, p1_duration), (p2, p2_duration))
}

fn hash_string(input: &str) -> usize {
    let mut current_value = 0;
    for c in input.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: isize,
}

impl Lens {
    fn new(input: &str) -> Self {
        let mut split = input.split(['-', '=']);
        let label = split.nth(0).unwrap().to_string();
        let focal_field = split.nth(0).unwrap_or("");

        if let Ok(focal_length) = focal_field.parse::<isize>() {
            Self {
                label,
                focal_length,
            }
        } else {
            Self {
                label,
                focal_length: -1,
            }
        }
    }
}

fn part_1(input: &str) -> usize {
    input.split(',').map(hash_string).sum()
}

#[inline(always)]
fn calc_box_power(lenses: &LinkedList<Lens>, box_number: usize) -> usize {
    lenses
        .iter()
        .enumerate()
        .map(|(slot, lens)| {
            let slot = slot + 1; // Lens slots are 1-indexed
            (1 + box_number) * slot * lens.focal_length as usize
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut boxes: Vec<LinkedList<Lens>> = Vec::with_capacity(257);
    // Boxes are 0-indexed.
    for _ in 0..256 {
        boxes.push(LinkedList::default());
    }

    for line in input.split(',') {
        let lens = Lens::new(line);
        let target_box = boxes.get_mut(hash_string(&lens.label)).unwrap();
        if line.contains('-') {
            target_box
                .extract_if(|l| l.label == lens.label)
                .for_each(drop);
        } else if line.contains('=') {
            if let Some(lens_to_replace) = target_box.iter_mut().find(|l| l.label == lens.label) {
                *lens_to_replace = lens;
            } else {
                target_box.push_back(lens);
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(box_number, lenses)| calc_box_power(lenses, box_number))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_1(input), 1320);
    }

    #[test]
    fn test_part_2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_2(input), 145);
    }
}
