use aoc2024::{input::get_all_numbers, sample};

fn main() {
    aoc2024::run(part1, None);
}

fn try_split_digits(value: usize) -> Option<(usize, usize)> {
    let ndigits = value.ilog10() + 1;
    if ndigits % 2 == 0 {
        let divisor = 10usize.pow(ndigits / 2);
        Some((value / divisor, value % divisor))
    } else {
        None
    }
}

fn expand_stones(stone: usize, depth: usize, target_depth: usize, acc: &mut Vec<usize>) {
    if depth == target_depth {
        acc.push(stone);
    } else if stone == 0 {
        expand_stones(1, depth + 1, target_depth, acc);
    } else if let Some((a, b)) = try_split_digits(stone) {
        expand_stones(a, depth + 1, target_depth, acc);
        expand_stones(b, depth + 1, target_depth, acc);
    } else {
        expand_stones(stone * 2024, depth + 1, target_depth, acc);
    }
}

fn part1(input: &str) -> String {
    let stones = get_all_numbers::<usize>(input);
    let mut acc = Vec::new();

    for stone in stones {
        expand_stones(stone, 0, 25, &mut acc);
    }

    acc.len().to_string()
}

sample! {
    r"125 17",
    part1 = "55312"
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test_log::test(rstest)]
    #[case(10, Some((1, 0)))]
    #[case(111, None)]
    #[case(1000, Some((10, 0)))]
    fn test_split_digits(#[case] value: usize, #[case] result: Option<(usize, usize)>) {
        assert_eq!(result, try_split_digits(value));
    }
}
