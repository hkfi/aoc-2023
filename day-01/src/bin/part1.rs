fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("Part 1: {}", result);
}

fn part1(input: &str) -> String {
    let output = input.lines().map(|line| {
        let mut chars = line.chars();
        let first = chars.find_map(|char| char.to_digit(10)).expect("should be number");
        let last = chars.rfind(|char| char.is_digit(10)).map(|char| char.to_digit(10).unwrap()).unwrap_or(first);

        first * 10 + last
    })
    .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(input);
        assert_eq!(result, "142");
    }
}