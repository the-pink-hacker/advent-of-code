const INPUT: &str = include_str!("../input");

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

fn is_report_safe(report: &[u8]) -> bool {
    assert!(report.len() >= 2);

    let report_delta = report[0].cmp(&report[1]);

    for i in 0..(report.len() - 1) {
        let level = report[i];
        let next_level = report[i + 1];

        let distance = level.abs_diff(next_level);
        let delta = level.cmp(&next_level);

        // Change in direction
        if delta != report_delta {
            return false;
        }

        match distance {
            1..=3 => (),
            _ => return false,
        }
    }

    true
}

fn part_one(reports: &[Vec<u8>]) {
    let total_safe = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();

    println!();
    println!("Part 1");
    println!("Safe Reports: {}", total_safe);
}

fn main() {
    let reports = parse_reports(INPUT);

    println!("=== Day 2 ===");

    part_one(&reports);
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
}
