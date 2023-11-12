use std::fs;

fn read_lines(filename: &str) -> Vec<usize> {
    fs::read_to_string(filename)
        .expect("File not readable")
        .trim_end()
        .split(",")
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn solve(noun: usize, verb: usize) -> usize {
    let mut lines = read_lines("full_input.txt");

    let n = lines.len();
    let mut index = 0;
    lines[1] = noun;
    lines[2] = verb;
    while index < n {
        let op = lines[index];
        if op == 99 {
            break;
        }
        let r1 = lines[index + 1];
        let r2 = lines[index + 2];
        let ind = lines[index + 3];
        lines[ind] = match op {
            1 => lines[r1] + lines[r2],
            2 => lines[r1] * lines[r2],
            _ => 0,
        };

        index += 4;
    }

    return lines[0];
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Part 1
    println!("Part 1");
    println!("Solution for part1: {}", solve(12, 2));

    // Part 2
    println!("Part 2");

    let mut noun = 0;
    let mut verb = 0;
    while noun <= 99 && solve(noun, verb) != 19690720 {
        noun += 1;
        verb = 0;
        while verb <= 99 && solve(noun, verb) != 19690720 {
            verb += 1;
        }
    }

    println!("Solution for part2: {}", 100 * noun + verb);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
