use std::{cmp, collections::HashMap};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap().lines().collect::<Vec<_>>();
    let test_list = split.next().unwrap().lines();

    let mut rules_map = HashMap::new();
    for rule in rules {
        let mut split = rule.split("|");
        let num_1 = split.next().unwrap();
        let num_2 = split.next().unwrap();
        rules_map.entry(num_1).or_insert_with(Vec::new).push(num_2);
    }

    let ret = test_list
        .map(|pages| pages.split(",").map(|x| x.parse::<u32>().unwrap()))
        .filter(|pages| {
            pages.clone().is_sorted_by(|a, b| {
                rules_map
                    .get(a.to_string().as_str())
                    .is_some_and(|x| x.contains(&b.to_string().as_str()))
            })
        })
        .map(|pages| {
            let page_vec = pages.collect::<Vec<_>>();
            page_vec[page_vec.len() / 2]
        })
        .sum::<u32>();
    Some(ret)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap().lines().collect::<Vec<_>>();
    let test_list = split.next().unwrap().lines();

    let mut rules_map = HashMap::new();
    for rule in rules {
        let mut split = rule.split("|");
        let num_1 = split.next().unwrap();
        let num_2 = split.next().unwrap();
        rules_map.entry(num_1).or_insert_with(Vec::new).push(num_2);
    }

    let ret = test_list
        .map(|pages| pages.split(",").map(|x| x.parse::<u32>().unwrap()))
        .filter(|pages| {
            !pages.clone().is_sorted_by(|a, b| {
                rules_map
                    .get(a.to_string().as_str())
                    .is_some_and(|x| x.contains(&b.to_string().as_str()))
            })
        })
        .map(|pages| {
            let mut page_vec = pages.collect::<Vec<_>>();
            page_vec.sort_by(|a, b| {
                match rules_map
                    .get(a.to_string().as_str())
                    .is_some_and(|x| x.contains(&b.to_string().as_str()))
                {
                    true => cmp::Ordering::Less,
                    _ => cmp::Ordering::Equal,
                }
            });
            page_vec[page_vec.len() / 2]
        })
        .sum::<u32>();
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
