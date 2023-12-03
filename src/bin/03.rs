use regex::Regex;
use std::ops::Range;

advent_of_code::solution!(3);

fn get_line_length(input: &str) -> usize {
    // account for newline character
    input.lines().next().unwrap().len()
}

fn get_part_numbers(input: &str) -> Vec<(Range<usize>, u32)> {
    let numbers_regex = Regex::new(r"[\d]+").unwrap();
    numbers_regex
        .find_iter(&input)
        .map(|m| {
            (
                m.range(),
                m.as_str()
                    .parse()
                    .expect(&format!("Couldn't parse {} as u32", m.as_str())),
            )
        })
        .collect()
}

fn is_symbol(input: &char) -> bool {
    input != &'.' && !input.is_digit(10)
}

fn neighbors(input: &str, index_range: &Range<usize>, line_length: &usize) -> Vec<char> {
    let mut neighbors: Vec<char> = vec![];
    let input: Vec<char> = input.chars().collect();
    let i_line_length = *line_length as i32;
    for index in index_range.clone() {
        let i_index = index as i32;
        let mut neighbor_indexes = vec![
            i_index - 1,
            i_index + 1,
            i_index - i_line_length,
            i_index + i_line_length,
        ];
        // start of line
        if !(index % line_length == 0) {
            neighbor_indexes.push(i_index + i_line_length - 1);
            neighbor_indexes.push(i_index - i_line_length - 1);
        }
        // enf of line
        if !(index % line_length == line_length - 1) {
            neighbor_indexes.push(i_index + i_line_length + 1);
            neighbor_indexes.push(i_index - i_line_length + 1);
        }
        for neighbor_index in neighbor_indexes {
            // boundary check
            if neighbor_index > 0 && neighbor_index < input.len() as i32 {
                neighbors.push(input[neighbor_index as usize])
            }
        }
    }
    neighbors.sort();
    neighbors.dedup();
    neighbors
}

pub fn part_one(input: &str) -> Option<u32> {
    let line_length = get_line_length(input);
    let input = input.replace("\n", "");
    let part_numbers = get_part_numbers(&input);

    Some(
        part_numbers
            .iter()
            .filter(|(index_range, _)| {
                neighbors(&input, index_range, &line_length)
                    .iter()
                    .any(|c| is_symbol(c))
            })
            .map(|(_, part_number)| part_number)
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
