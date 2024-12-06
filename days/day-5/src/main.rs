use std::collections::HashMap;

const INPUT: &str = include_str!("../input");

fn parse_input(raw: &str) -> (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let (orders, pages) = raw.split_once("\n\n").unwrap();

    let orders = orders.lines().map(|line| {
        let (before, after) = line.split_once('|').unwrap();
        (before.parse::<u8>().unwrap(), after.parse::<u8>().unwrap())
    });

    let mut order_table = HashMap::<u8, Vec<u8>>::new();

    for (before, after) in orders {
        order_table.entry(before).or_default().push(after);
    }

    let pages = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    (order_table, pages)
}

fn seek<'a, I>(after_values: &[u8], page: &I) -> bool
where
    I: Iterator<Item = &'a u8>,
{
    for after in after_values {
        let x = page.by_ref().find(|x| *x == after);
    }

    false
}

fn part_one(orders: &HashMap<u8, Vec<u8>>, pages: &[Vec<u8>]) -> u32 {
    let mut output = 0;

    'page: for page in pages {
        let mut page_iter = page.iter();

        while let Some(before) = page_iter.next() {
            if let Some(after) = orders.get(before) {
                if !seek(after, &page_iter) {
                    dbg!(before, after);
                    continue 'page;
                }
            }
        }

        let middle_value = page[page.len() / 2];
        dbg!(middle_value);
        output += middle_value as u32;
    }

    output
}

fn main() {
    let (orders, pages) = parse_input(INPUT);

    println!("=== Day 5 ===");

    println!();
    println!("Part One:");
    println!("{}", part_one(&orders, &pages));
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn example_1() {
        let (orders, pages) = parse_input(EXAMPLE_ONE);
        assert_eq!(part_one(&orders, &pages), 143);
    }
}
