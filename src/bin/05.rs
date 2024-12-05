use std::rc::Rc;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap().lines().collect::<Vec<_>>();
    let test_list = split.next().unwrap().lines();

    let ret = test_list
        .map(|pages| {
            let page_vec = pages.split(",").collect::<Vec<_>>();
            let mut needed_rules = Vec::new();
            for i in 0..page_vec.len() - 1 {
                for j in i + 1..page_vec.len() {
                    needed_rules.push(format!("{}|{}", page_vec[i], page_vec[j]));
                }
            }
            for rule in needed_rules {
                if !rules.contains(&rule.as_str()) {
                    return 0;
                }
            }

            page_vec[page_vec.len() / 2].parse::<i32>().unwrap()
        })
        .sum::<i32>();
    Some(ret as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut split = input.split("\n\n");
    let rules = Rc::new(split.next().unwrap().lines().collect::<Vec<_>>());
    let test_list = split.next().unwrap().lines();

    let ret = test_list
        .map(|pages| {
            let page_vec = pages.split(",").collect::<Vec<_>>();
            let mut needed_rules = Vec::new();
            for i in 0..page_vec.len() - 1 {
                for j in i + 1..page_vec.len() {
                    needed_rules.push(format!("{}|{}", page_vec[i], page_vec[j]));
                }
            }
            for rule in needed_rules {
                if !rules.contains(&rule.as_str()) {
                    let corrected_page_order = correct_page_order(&page_vec, rules.clone());
                    return corrected_page_order[page_vec.len() / 2]
                        .parse::<i32>()
                        .unwrap();
                }
            }

            0
        })
        .sum::<i32>();
    Some(ret as u32)
}

pub fn correct_page_order(pages: &Vec<&str>, rules: Rc<Vec<&str>>) -> Vec<String> {
    let mut needed_rules = Vec::new();
    for i in 0..pages.len() - 1 {
        for j in i + 1..pages.len() {
            needed_rules.push(format!("{}|{}", pages[i], pages[j]));
        }
    }
    for rule in needed_rules {
        if !rules.contains(&rule.as_str()) {
            let mut split = rule.split("|");
            let num_1 = split.next().unwrap();
            let num_2 = split.next().unwrap();
            let mut new_pages = pages.clone();
            new_pages.swap(
                pages.iter().position(|num| *num == num_1).unwrap(),
                pages.iter().position(|num| *num == num_2).unwrap(),
            );
            return correct_page_order(&new_pages, rules);
        }
    }

    pages.iter().map(|s| s.to_string()).collect::<Vec<String>>()
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
