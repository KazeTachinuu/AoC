use std::fs;

fn read_lines(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect("File not readable")
        .lines()
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn req(mut n: i32) -> i32 {
    let mut total: i32 = 0;
    while n > 6 {
        n = n / 3 - 2;
        total += n;
    }
    total
}
fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let lines = read_lines("full_input.txt");

    // println!("{:?}", lines);

    // Part 1
    println!("Part 1");

    let part1: i32 = lines.iter().map(|l| l / 3 - 2).sum();

    let part2: i32 = lines.iter().map(|l| req(*l)).sum();

    println!("Answer part 1: {:?}", part1);

    // Part 2
    println!("Part 2");

    println!("Answer part 2: {:?}", part2);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
