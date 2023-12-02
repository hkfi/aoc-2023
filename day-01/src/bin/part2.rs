fn main() {
    let input = include_str!("./input1.txt");
    let result = part2(input);
    println!("Part 2: {}", result);
}

fn part2(input: &str) -> String {
    let output = input.lines().map(|line| {
        let mut nums = (0..line.len()).filter_map(|index| {
            let reduced_line = &line[index..];
            let result = match reduced_line {
                s if s.starts_with("one") => Some(1),
                s if s.starts_with("two") => Some(2),
                s if s.starts_with("three") => Some(3),
                s if s.starts_with("four") => Some(4),
                s if s.starts_with("five") => Some(5),
                s if s.starts_with("six") => Some(6),
                s if s.starts_with("seven") => Some(7),
                s if s.starts_with("eight") => Some(8),
                s if s.starts_with("nine") => Some(9),
                _ => reduced_line.chars().next().unwrap().to_digit(10),
            };

            result

        });

        let first = nums.next().expect("should be number");

        match nums.last() {
            Some(num) => first * 10 + num,
            None => first * 10 + first
        }
    })
    .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part2(input);
        assert_eq!(result, "281");
    }
}