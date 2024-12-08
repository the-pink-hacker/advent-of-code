use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../input");

type Table = HashMap<u8, Vec<u8>>;

fn parse_orders(orders: &str) -> Table {
    let orders = orders.lines().map(|line| {
        let (before, after) = line.split_once('|').unwrap();
        (before.parse::<u8>().unwrap(), after.parse::<u8>().unwrap())
    });

    let mut after_table = HashMap::<u8, Vec<u8>>::new();

    for (before, after) in orders {
        after_table.entry(before).or_default().push(after);
    }

    after_table
}

fn parse_input(raw: &str) -> (Table, Vec<Vec<u8>>) {
    let (orders, pages) = raw.split_once("\n\n").unwrap();

    let after_table = parse_orders(orders);

    let pages = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    (after_table, pages)
}

fn check_page(after_table: &Table, page: &[u8]) -> Option<u8> {
    for pages in page.iter().combinations(2) {
        if after_table
            .get(pages[1])
            .map(|values| values.contains(pages[0]))
            .unwrap_or_default()
        {
            return None;
        }
    }

    Some(page[page.len() / 2])
}

fn part_one(after_table: &Table, pages: &[Vec<u8>]) -> u32 {
    pages
        .iter()
        .filter_map(|page| check_page(after_table, page))
        .map(u32::from)
        .sum()
}

fn main() {
    let (after_table, pages) = parse_input(INPUT);

    println!("=== Day 5 ===");

    println!();
    println!("Part One:");
    println!("{}", part_one(&after_table, &pages));
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
        let after_table = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[75, 47, 61, 53, 29];
        assert_eq!(check_page(&after_table, page), Some(61));
    }

    #[test]
    fn example_1_1() {
        let after_table = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[97, 61, 53, 29, 13];
        assert_eq!(check_page(&after_table, page), Some(53));
    }

    #[test]
    fn example_1_2() {
        let after_table = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[75, 29, 13];
        assert_eq!(check_page(&after_table, page), Some(29));
    }

    #[test]
    fn example_1_3() {
        let after_table = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[75, 97, 47, 61, 43];
        assert_eq!(check_page(&after_table, page), None);
    }

    #[test]
    fn example_1_4() {
        let after_table = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[61, 13, 29];
        assert_eq!(check_page(&after_table, page), None);
    }

    #[test]
    fn example_1_5() {
        let after_table = parse_orders(EXAMPLE_ONE_ORDERS);
        let page = &[97, 13, 75, 29, 47];
        assert_eq!(check_page(&after_table, page), None);
    }

    #[test]
    fn example_1() {
        let (after_table, pages) = parse_input(EXAMPLE_ONE);
        assert_eq!(part_one(&after_table, &pages), 143);
    }

    #[test]
    fn part_one_final() {
        let (after_table, pages) = parse_input(INPUT);
        assert_eq!(part_one(&after_table, &pages), 7307);
    }
}
