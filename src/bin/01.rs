advent_of_code::solution!(1);

fn numbers_in_lines(input: &String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        numbers_in_lines(&input.to_string())
            .iter()
            .map(|numbers| {
                let first = numbers.first().expect("No numbers found in line");
                let second = numbers.last().expect("No numbers found in line");
                format!("{first}{second}").parse::<u32>().unwrap()
            })
            .sum(),
    )
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn numbers_in_lines_2(line: &str) -> Vec<u32> {
    let mut number_indices: Vec<(u32, u32)> = vec![];
    for (number_index, number_str) in NUMBERS.iter().enumerate() {
        let number = number_index + 1;
        line.match_indices(number_str)
            .for_each(|(index, _)| number_indices.push((index as u32, number as u32)));
    }
    for digit in '0'..='9' {
        line.match_indices(digit).for_each(|(index, _)| {
            number_indices.push((index as u32, digit.to_digit(10).unwrap() as u32))
        });
    }
    number_indices.sort_by(|(index1, _), (index2, _)| index1.cmp(index2));
    number_indices.iter().map(|(_, number)| *number).collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| numbers_in_lines_2(line))
            .map(|numbers| {
                let first = numbers.first().expect("No numbers found in line");
                let second = numbers.last().expect("No numbers found in line");
                format!("{first}{second}").parse::<u32>().unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
