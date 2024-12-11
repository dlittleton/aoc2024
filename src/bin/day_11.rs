use aoc2024::{input::get_all_numbers, sample};
use cached::{proc_macro::cached, Cached};
use tracing::info;

fn main() {
    aoc2024::run(part1, Some(part2));
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

#[cached]
fn expand_stones(stone: usize, depth: usize, target_depth: usize) -> usize {
    if depth == target_depth {
        1
    } else if stone == 0 {
        expand_stones(1, depth + 1, target_depth)
    } else if let Some((a, b)) = try_split_digits(stone) {
        expand_stones(a, depth + 1, target_depth) + expand_stones(b, depth + 1, target_depth)
    } else {
        expand_stones(stone * 2024, depth + 1, target_depth)
    }
}

fn part1(input: &str) -> String {
    let stones = get_all_numbers::<usize>(input);

    let total: usize = stones.iter().map(|s| expand_stones(*s, 0, 25)).sum();

    if let Ok(expand_cache) = EXPAND_STONES.try_lock() {
        info!("Cache size {}", expand_cache.cache_size());
        info!(
            "Cache hits {}",
            expand_cache.cache_hits().unwrap_or_default()
        );
    }

    total.to_string()
}

fn part2(input: &str) -> String {
    let stones = get_all_numbers::<usize>(input);

    let total: usize = stones.iter().map(|s| expand_stones(*s, 0, 75)).sum();

    if let Ok(expand_cache) = EXPAND_STONES.try_lock() {
        info!("Cache size {}", expand_cache.cache_size());
        info!(
            "Cache hits {}",
            expand_cache.cache_hits().unwrap_or_default()
        );
    }

    total.to_string()
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
