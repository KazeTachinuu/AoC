use std::fs;

fn read_lines(filename: &str) -> Vec<u64> {
    fs::read_to_string(filename)
        .expect("File not readable")
        .lines()
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let lines = read_lines("test_input.txt");

    println!("{:?}", lines);

    // Part 1
    println!("Part 1");

    println!("Answer part 1: {:?}", "");

    // Part 2
    println!("Part 2");

    println!("Answer part 2: {:?}", "");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
