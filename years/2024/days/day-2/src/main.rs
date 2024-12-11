use std::cmp::Ordering;

use common::*;

include_input!(INPUT);

fn parse_reports(data: &str) -> Vec<Vec<u8>> {
    data.lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn is_level_safe(level: u8, next_level: u8, report_delta: Ordering) -> bool {
    let delta = level.cmp(&next_level);

    // Change in direction
    if delta != report_delta {
        return false;
    }

    let distance = level.abs_diff(next_level);

    matches!(distance, 1..=3)
}

fn is_report_safe(report: &[u8]) -> bool {
    assert!(report.len() >= 2);

    let report_delta = report[0].cmp(&report[1]);

    for i in 0..(report.len() - 1) {
        let level = report[i];
        let next_level = report[i + 1];

        if !is_level_safe(level, next_level, report_delta) {
            return false;
        }
    }

    true
}

fn is_report_mostly_safe(report: &[u8]) -> bool {
    if is_report_safe(report) {
        true
    } else {
        for i in 0..report.len() {
            let mut report = report.to_vec();
            report.remove(i);
            if is_report_safe(&report) {
                return true;
            }
        }

        false
    }
}

fn part_one(reports: &[Vec<u8>]) -> usize {
    reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count()
}

fn part_two(reports: &[Vec<u8>]) -> usize {
    reports
        .iter()
        .filter(|report| is_report_mostly_safe(report))
        .count()
}

fn main() {
    let reports = parse_reports(INPUT);

    advent_solution(2024, 2, part_one(&reports), part_two(&reports));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_safe_1() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_report_safe(&report));
    }

    #[test]
    fn example_safe_2() {
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_report_safe(&report));
    }

    #[test]
    fn example_safe_3() {
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_report_safe(&report));
    }

    #[test]
    fn example_safe_4() {
        let report = vec![1, 3, 2, 4, 5];
        assert!(!is_report_safe(&report));
    }

    #[test]
    fn example_safe_5() {
        let report = vec![8, 6, 4, 4, 1];
        assert!(!is_report_safe(&report));
    }

    #[test]
    fn example_safe_6() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_report_safe(&report));
    }

    #[test]
    fn example_mostly_safe_1() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_report_mostly_safe(&report));
    }

    #[test]
    fn example_mostly_safe_2() {
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_report_mostly_safe(&report));
    }

    #[test]
    fn example_mostly_safe_3() {
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_report_mostly_safe(&report));
    }

    #[test]
    fn example_mostly_safe_4() {
        let report = vec![1, 3, 2, 4, 5];
        assert!(is_report_mostly_safe(&report));
    }

    #[test]
    fn example_mostly_safe_5() {
        let report = vec![8, 6, 4, 4, 1];
        assert!(is_report_mostly_safe(&report));
    }

    #[test]
    fn example_mostly_safe_6() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_report_mostly_safe(&report));
    }

    #[test]
    fn exmaple_mostly_safe_first_fail() {
        let report = vec![10, 9, 11, 12, 13, 14];
        assert!(is_report_mostly_safe(&report));
    }

    #[test]
    fn part_one_final() {
        let reports = parse_reports(INPUT);
        assert_eq!(part_one(&reports), 287);
    }

    #[test]
    fn part_two_final() {
        let reports = parse_reports(INPUT);
        assert_eq!(part_two(&reports), 354);
    }
}
