use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    part1();
    part2();
}

fn get_numbers() -> Vec<u32> {
    read_to_string("input.txt")
        .unwrap()
        .split("\r\n")
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn part1() {
    let mut numbers = get_numbers();

    numbers.sort();

    // index from low end
    let mut i = 0;
    // index from high end
    let mut j = numbers.len() - 1;

    loop {
        let sum = numbers[i] + numbers[j];
        if sum < 2020 {
            // if sum is smaller than we increase by the next smallest number
            i += 1;
        } else if sum > 2020 {
            // if sum is larger then we decrease by the larger number
            j -= 1;
        } else if sum == 2020 {
            println!(
                "{} + {} = {}, {} * {} = {}",
                numbers[i],
                numbers[j],
                numbers[i] + numbers[j],
                numbers[i],
                numbers[j],
                numbers[i] * numbers[j]
            );
            break;
        } else if j <= i {
            println!("No solution");
            break;
        }
    }
}

fn part2() {
    let mut numbers = get_numbers();
    numbers.sort();

    let solution = numbers
        .iter()
        .permutations(3)
        .filter(|v| v[0] + v[1] + v[2] == 2020)
        .map(|v| (*v[0], *v[1], *v[2]))
        .next()
        .unwrap();

    println!(
        "{} + {} + {} = {}, {} * {} * {} = {}",
        solution.0,
        solution.1,
        solution.2,
        solution.0 + solution.1 + solution.2,
        solution.0,
        solution.1,
        solution.2,
        solution.0 * solution.1 * solution.2,
    )
}
