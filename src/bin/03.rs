use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let num_regex = Regex::new(r"[0-9]+,[0-9]+").unwrap();
    let matches = regex.find_iter(input);
    let result = matches.map(|m| {
        let num_string = num_regex.find(m.as_str()).unwrap().as_str();
        let nums = num_string.split(',').collect::<Vec<_>>();
        println!("{:?}", nums);
        nums[0].parse::<u32>().unwrap() * nums[1].parse::<u32>().unwrap()
    });
    Some(result.sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let num_regex = Regex::new(r"[0-9]+,[0-9]+").unwrap();
    let do_dont_regex = Regex::new(r"do\(\)|don\'t\(\)").unwrap();
    let do_dont_matches = do_dont_regex.find_iter(input).collect::<Vec<_>>();
    let matches = regex.find_iter(input);
    let result = matches.map(|m| {
        let prev_cmd = do_dont_matches
            .clone()
            .into_iter()
            .filter(|cmd| cmd.start() < m.start())
            .last();
        if prev_cmd.is_some() && prev_cmd.unwrap().as_str() == "don't()" {
            return 0;
        }

        let num_string = num_regex.find(m.as_str()).unwrap().as_str();
        let nums = num_string.split(',').collect::<Vec<_>>();
        println!("{:?}", nums);
        nums[0].parse::<u32>().unwrap() * nums[1].parse::<u32>().unwrap()
    });
    Some(result.sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
