use aoc21::utils;
use itertools::Itertools;

type MyResult<T> = std::result::Result<T, String>;

fn main() -> MyResult<()> {
    let input = utils::get_input().unwrap();

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &str) -> MyResult<u64> {
    Ok(0)
}

fn exo2(input: &str) -> MyResult<u64> {
    Ok(0)
}
