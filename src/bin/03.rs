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

fn neighbors_single(input: &str, index: &usize, line_length: &usize) -> Vec<usize> {
    let mut neighbors: Vec<usize> = vec![];
    let i_line_length = *line_length as i32;
    let i_index = *index as i32;
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
            neighbors.push(neighbor_index as usize)
        }
    }
    neighbors
}

fn neighbors_range(input: &str, index_range: &Range<usize>, line_length: &usize) -> Vec<usize> {
    let mut neighbors: Vec<usize> = vec![];
    for index in index_range.clone() {
        neighbors.extend(neighbors_single(input, &index, line_length));
    }
    neighbors.sort();
    neighbors.dedup();
    neighbors
}

pub fn part_one(input: &str) -> Option<u32> {
    let line_length = get_line_length(input);
    let input = input.replace("\n", "");
    let input_chars: Vec<char> = input.chars().collect();
    let part_numbers = get_part_numbers(&input);

    Some(
        part_numbers
            .iter()
            .filter(|(index_range, _)| {
                neighbors_range(&input, index_range, &line_length)
                    .iter()
                    .any(|&index| is_symbol(&input_chars[index]))
            })
            .map(|(_, part_number)| part_number)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_length = get_line_length(input);
    let input = input.replace("\n", "");
    let part_numbers = get_part_numbers(&input);
    let part_numbers_neighbors: Vec<(Vec<usize>, u32)> = part_numbers
        .iter()
        .map(|(part_number_range, part_number)| {
            (
                neighbors_range(&input, part_number_range, &line_length),
                *part_number,
            )
        })
        .collect();
    let gears: Vec<usize> = input.match_indices('*').map(|(index, _)| index).collect();

    Some(
        gears
            .iter()
            .map(|gear_index| {
                // Ugly hack to not have to duplicate neighbors
                let mut neighboring_numbers: Vec<u32> = vec![];
                for (neighbors, part_number) in &part_numbers_neighbors {
                    if neighbors.contains(&gear_index) {
                        neighboring_numbers.push(*part_number);
                    }
                }
                if neighboring_numbers.len() == 2 {
                    neighboring_numbers[0] * neighboring_numbers[1]
                } else {
                    0
                }
            })
            .sum(),
    )
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
        assert_eq!(result, Some(467835));
    }
}
