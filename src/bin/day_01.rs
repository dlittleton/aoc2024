use aoc2024::input::get_all_numbers;

fn main() {
    aoc2024::run(part1, None);
}

fn part1(input: &str) -> String {
    let mut first = Vec::new();
    let mut second = Vec::new();

    input.split('\n').for_each(|l| {
        let nums = get_all_numbers::<i32>(l);
        first.push(*nums.first().unwrap());
        second.push(*nums.last().unwrap());
    });

    first.sort();
    second.sort();

    let total: i32 = first
        .into_iter()
        .zip(second)
        .map(|(a, b)| (a - b).abs())
        .sum();

    format!("{}", total)
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        let contents = SAMPLE.trim_start();
        let result = part1(contents);
        assert_eq!("11", result);
    }
}
