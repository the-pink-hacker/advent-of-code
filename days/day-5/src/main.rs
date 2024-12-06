use std::collections::HashMap;

const INPUT: &str = include_str!("../input");

type Table = HashMap<u8, Vec<u8>>;

fn parse_orders(orders: &str) -> (Table, Table) {
    let orders = orders.lines().map(|line| {
        let (before, after) = line.split_once('|').unwrap();
        (before.parse::<u8>().unwrap(), after.parse::<u8>().unwrap())
    });

    let mut before_table = HashMap::<u8, Vec<u8>>::new();
    let mut after_table = HashMap::<u8, Vec<u8>>::new();

    for (before, after) in orders {
        before_table.entry(after).or_default().push(before);
        after_table.entry(before).or_default().push(after);
    }

    (before_table, after_table)
}

fn parse_input(raw: &str) -> (Table, Table, Vec<Vec<u8>>) {
    let (orders, pages) = raw.split_once("\n\n").unwrap();

    let (before_table, after_table) = parse_orders(orders);

    let pages = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    (before_table, after_table, pages)
}

fn seek(values: &[u8], page: &[u8]) -> bool {
    for value in values {
        if !page.contains(value) {
            return false;
        }
    }

    true
}

fn check_page(before_table: &Table, after_table: &Table, page: &[u8]) -> Option<u8> {
    for (i, value) in page.iter().enumerate() {
        if let Some(before) = before_table.get(value) {
            if seek(before, &page[i + 1..]) {
                // Before value was found after value
                return None;
            } else {
                dbg!("BEFORE", value, before, page);
            }
        }

        if let Some(after) = after_table.get(value) {
            if seek(after, &page[..i]) {
                // After value was found before value
                return None;
            }
        }
    }

    Some(page[page.len() / 2])
}

fn part_one(before_table: &Table, after_table: &Table, pages: &[Vec<u8>]) -> u32 {
    pages
        .iter()
        .filter_map(|page| check_page(before_table, after_table, page))
        .map(u32::from)
        .sum()
}

fn main() {
    let (before_table, after_table, pages) = parse_input(INPUT);

    println!("=== Day 5 ===");

    println!();
    println!("Part One:");
    println!("{}", part_one(&before_table, &after_table, &pages));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE_ORDERS: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";

    const EXAMPLE_ONE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn example_1_0() {
        let (before_table, after_table) = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[75, 47, 61, 53, 29];
        assert_eq!(check_page(&before_table, &after_table, page), Some(61));
    }

    #[test]
    fn example_1_1() {
        let (before_table, after_table) = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[97, 61, 53, 29, 13];
        assert_eq!(check_page(&before_table, &after_table, page), Some(53));
    }

    #[test]
    fn example_1_2() {
        let (before_table, after_table) = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[75, 29, 13];
        assert_eq!(check_page(&before_table, &after_table, page), Some(29));
    }

    #[test]
    fn example_1_3() {
        let (before_table, after_table) = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[75, 97, 47, 61, 43];
        assert_eq!(check_page(&before_table, &after_table, page), None);
    }

    #[test]
    fn example_1_4() {
        let (before_table, after_table) = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[61, 13, 29];
        assert_eq!(check_page(&before_table, &after_table, page), None);
    }

    #[test]
    fn example_1_5() {
        let (before_table, after_table) = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[97, 13, 75, 29, 47];
        assert_eq!(check_page(&before_table, &after_table, page), None);
    }

    #[test]
    fn example_1() {
        let (before_table, after_table, pages) = parse_input(EXAMPLE_ONE);
        assert_eq!(part_one(&before_table, &after_table, &pages), 143);
    }

    #[test]
    fn part_one_final() {
        let (before_table, after_table, pages) = parse_input(INPUT);
        // 46|62
        let test = &[vec![32, 99, 27, 62, 48, 64, 46, 98, 14]];
        assert_eq!(part_one(&before_table, &after_table, test), 0);
    }
}
