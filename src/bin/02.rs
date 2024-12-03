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
                        if diff < 1 || diff > 3 {
                            safe = false
                        }
                        if next_num > num {
                            increasing = true
                        }
                    }

                    if i < nums.len() - 1 {
                        let next_num = nums[i + 1].1;
                        let diff = (num - next_num).abs();
                        if diff < 1 || diff > 3 {
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
                return 0;
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
