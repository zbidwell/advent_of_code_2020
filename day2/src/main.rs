use regex::Regex;

fn main() {
    let lines = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\r\n")
        .map(str::to_string)
        .collect::<Vec<String>>();

    let part1 = part1(lines.clone());
    let part2 = part2(lines);

    println!("part 1: {}, part 2: {}", part1, part2);
}

fn part1(lines: Vec<String>) -> usize {
    lines.into_iter().filter(|s| validate1(s)).count()
}

fn part2(lines: Vec<String>) -> usize {
    lines.into_iter().filter(|s| validate2(s)).count()
}

fn validate1(line: &str) -> bool {
    let re = Regex::new(r#"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)"#).unwrap();
    let captures = re.captures(line).unwrap();

    let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let letter = captures.get(3).unwrap().as_str().chars().next().unwrap();
    let password = captures.get(4).unwrap().as_str();

    let count = password.chars().filter(|c| *c == letter).count();
    count >= min && count <= max
}

fn validate2(line: &str) -> bool {
    let re = Regex::new(r#"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)"#).unwrap();
    let captures = re.captures(line).unwrap();

    let i = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let j = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let letter = captures.get(3).unwrap().as_str().chars().next().unwrap();
    let password = captures
        .get(4)
        .unwrap()
        .as_str()
        .chars()
        .collect::<Vec<char>>();

    (password[i - 1] == letter) ^ (password[j - 1] == letter)
}
