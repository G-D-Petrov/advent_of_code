use std::ops::RangeInclusive;

#[allow(dead_code)]
fn check_range_partial(bigger: RangeInclusive<usize>, smaller: RangeInclusive<usize>) -> bool {
    bigger.contains(smaller.start()) || bigger.contains(smaller.end())
}

#[allow(dead_code)]
fn check_range_full(bigger: RangeInclusive<usize>, smaller: RangeInclusive<usize>) -> bool {
    bigger.contains(smaller.start()) && bigger.contains(smaller.end())
}

fn parse_line(line: &str) -> (usize, RangeInclusive<usize>, usize, RangeInclusive<usize>) {
    let ranges = line.split(',').collect::<Vec<_>>();
    let bounds1 = ranges[0].split('-').collect::<Vec<_>>();
    let bounds2 = ranges[1].split('-').collect::<Vec<_>>();
    let size1 = bounds1[1].parse::<usize>().unwrap() - bounds1[0].parse::<usize>().unwrap();
    let size2 = bounds2[1].parse::<usize>().unwrap() - bounds2[0].parse::<usize>().unwrap();
    let range1 = bounds1[0].parse::<usize>().unwrap()..=bounds1[1].parse::<usize>().unwrap();
    let range2 = bounds2[0].parse::<usize>().unwrap()..=bounds2[1].parse::<usize>().unwrap();
    (size1, range1, size2,range2)
}

pub fn camp_cleanup(input: &str, check_range: fn(RangeInclusive<usize>, RangeInclusive<usize>) -> bool) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let (size1, range1, size2, range2) = parse_line(line);
        result += if size1 > size2 {
            check_range(range1, range2) as usize
        } else {
            check_range(range2, range1) as usize
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_1() {
        let input = include_str!("test_input.txt");
        let result = camp_cleanup(input, check_range_full);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_1_2() {
        let input = include_str!("test_1_2.txt");
        let result = camp_cleanup(input, check_range_full);
        assert_eq!(result, 464);
    }

    #[test]
    fn part_2_1() {
        let input = include_str!("test_input.txt");
        let result = camp_cleanup(input, check_range_partial);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_2_2() {
        let input = include_str!("test_2_2.txt");
        let result = camp_cleanup(input, check_range_partial);
        assert_eq!(result, 770);
    }
}
