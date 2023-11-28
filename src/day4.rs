use std::{fs, io::Result};

fn main() -> Result<()> {
    // let content = fs::read_to_string("./data/4.example.txt")?;
    let content = fs::read_to_string("./data/4.txt")?;

    let total: Vec<usize> = content
        .trim_end()
        .split("\n")
        .map(|line| {
            let (first, second): (&str, &str) = line.split_once(",").unwrap();

            let (first_start, first_end): (usize, usize) = match first.split_once("-") {
                Some((s, e)) => (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()),
                None => (0, 0),
            };

            let (second_start, second_end): (usize, usize) = match second.split_once("-") {
                Some((s, e)) => (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()),
                None => (0, 0),
            };

            let mut result = 0;
            if first_start <= second_start && first_end >= second_end {
                result = 1;
            } else if second_start <= first_start && second_end >= first_end {
                result = 1;
            }

            println!(
                "{result} - {first_start}-{first_end},{second_start}-{second_end} ----- {line}"
            );
            return result;
        })
        .collect();

    let sum: usize = total.iter().sum();
    println!("{:?}", sum);

    return Ok(());
}
