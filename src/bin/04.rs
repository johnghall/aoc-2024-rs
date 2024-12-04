use std::{char, rc::Rc};

advent_of_code::solution!(4);

#[derive(Debug)]
pub struct PointInfo {
    column_len: isize,
    row_len: isize,
    x: isize,
    y: isize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = Rc::new(input.lines().collect::<Vec<_>>());
    let column_len = lines.len();
    let row_len = lines[0].len();
    let mut total_xmas_count = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            let xmas_count = match letter {
                'X' => check_x_for_xmas(
                    PointInfo {
                        column_len: column_len as isize,
                        row_len: row_len as isize,
                        x: x as isize,
                        y: y as isize,
                    },
                    lines.clone(),
                ),
                _ => 0,
            };

            total_xmas_count += xmas_count;
        }
    }

    Some(total_xmas_count as u32)
}

pub fn check_x_for_xmas(start_point: PointInfo, lines: Rc<Vec<&str>>) -> i32 {
    let mut count = 0;
    for x_step in -1..=1 {
        for y_step in -1..=1 {
            let next_3_letters = get_next_n_letters(
                start_point.x,
                start_point.y,
                x_step,
                y_step,
                3,
                lines.clone(),
            );
            match next_3_letters.as_slice() {
                ['M', 'A', 'S'] => count += 1,
                _ => {}
            }
        }
    }

    count
}

pub fn get_next_n_letters(
    start_x: isize,
    start_y: isize,
    x_step: isize,
    y_step: isize,
    n: isize,
    lines: Rc<Vec<&str>>,
) -> Vec<char> {
    let mut letters = Vec::new();
    for i in 1..=n {
        let new_x = start_x + i * x_step;
        let new_y = start_y + i * y_step;
        match get_letter_at_point(
            PointInfo {
                column_len: lines.len() as isize,
                row_len: lines[start_y as usize].len() as isize,
                x: new_x,
                y: new_y,
            },
            lines.clone(),
        ) {
            Some(letter) => letters.push(letter),
            None => {}
        }
    }
    letters
}

pub fn get_letter_at_point(info: PointInfo, lines: Rc<Vec<&str>>) -> Option<char> {
    if info.x < 0 || info.y < 0 || info.x >= info.row_len || info.y >= info.column_len {
        return None;
    }

    Some(lines[info.y as usize].as_bytes()[info.x as usize] as char)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = Rc::new(input.lines().collect::<Vec<_>>());
    let column_len = lines.len();
    let row_len = lines[0].len();
    let mut total_xmas_count = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            let xmas_count = match letter {
                'A' => check_a_for_xmas(
                    PointInfo {
                        column_len: column_len as isize,
                        row_len: row_len as isize,
                        x: x as isize,
                        y: y as isize,
                    },
                    lines.clone(),
                ),
                _ => 0,
            };

            total_xmas_count += xmas_count;
        }
    }

    Some(total_xmas_count as u32)
}

pub fn check_a_for_xmas(start_point: PointInfo, lines: Rc<Vec<&str>>) -> i32 {
    let mut letters = Vec::new();
    for x_step in [-1, 1] {
        for y_step in [-1, 1] {
            let mut letter = get_next_n_letters(
                start_point.x,
                start_point.y,
                x_step,
                y_step,
                1,
                lines.clone(),
            );
            letters.append(&mut letter);
        }
    }

    match letters.as_slice() {
        ['S', 'S', 'M', 'M'] => 1,
        ['M', 'M', 'S', 'S'] => 1,
        ['M', 'S', 'M', 'S'] => 1,
        ['S', 'M', 'S', 'M'] => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
