advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let nums = line
                    .split(' ')
                    .map(|num| num.parse::<i32>().unwrap())
                    .enumerate()
                    .collect::<Vec<_>>();

                let mut safe = true;
                let mut increasing = false;
                for (i, num) in nums.clone() {
                    if i == 0 {
                        let next_num = nums[i + 1].1;
                        let diff = (num - next_num).abs();
                        if !(1..=3).contains(&diff) {
                            safe = false
                        }
                        if next_num > num {
                            increasing = true
                        }
                    }

                    if i < nums.len() - 1 {
                        let next_num = nums[i + 1].1;
                        let diff = (num - next_num).abs();
                        if !(1..=3).contains(&diff) {
                            safe = false
                        }
                        if next_num > num {
                            safe = safe && increasing
                        } else if next_num < num {
                            safe = safe && !increasing
                        } else {
                            safe = false
                        }
                    }
                }

                if safe {
                    return 1;
                }
                0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut possible_lines = Vec::new();
                let line_vec = line
                    .split_whitespace()
                    .map(str::parse)
                    .filter_map(Result::ok)
                    .collect::<Vec<i32>>();
                for i in 0..line_vec.len() {
                    if let Some(new_line) = remove_one_index(line_vec.clone(), i) {
                        possible_lines.push(new_line);
                    }
                }

                if possible_lines.into_iter().any(|l| {
                    is_line_safe(&l.into_iter().enumerate().collect::<Vec<(usize, i32)>>())
                }) {
                    return 1;
                }

                0
            })
            .sum(),
    )
}

pub fn is_line_safe(line: &Vec<(usize, i32)>) -> bool {
    let mut increasing = true;
    for i in 0..line.len() - 1 {
        let cur = line[i].1;
        let next = line[i + 1].1;
        let diff = cur - next;

        let is_increasing = diff < 0;

        if i == 0 {
            increasing = is_increasing
        }

        if diff == 0 {
            return false;
        }

        if is_increasing != increasing {
            return false;
        }

        if !(1..=3).contains(&diff.abs()) {
            return false;
        }
    }
    true
}

pub fn remove_one_index(mut line: Vec<i32>, idx: usize) -> Option<Vec<i32>> {
    if idx >= line.len() {
        return None;
    }

    line.remove(idx);
    Some(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
