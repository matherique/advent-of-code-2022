use std::{fs, io::Result};

fn point(c: &char) -> usize {
    if c.is_lowercase() {
        let lower = "abcdefghijklmnopqrstuvwxyz";

        if let Some(index) = lower.find(*c) {
            return index + 1;
        }

        return 0;
    }

    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if let Some(index) = upper.find(*c) {
        return index + 27;
    }

    return 0;
}

fn main() -> Result<()> {
    // let input = fs::read_to_string("./data/3.example.txt")?;
    let input = fs::read_to_string("./data/3.txt")?;

    let parts: Vec<&str> = input.trim_end().split("\n").collect();

    let points: Vec<usize> = parts
        .chunks(3)
        .map(|lines| {
            if let Some(first) = lines.get(0) {
                for i in first.chars() {
                    if lines[1].contains(i) && lines[2].contains(i) {
                        return point(&i);
                    }
                }
            }

            return 0;
        })
        .collect();

    let sum: usize = points.iter().sum();
    println!("{:?}", sum);

    Ok(())
}
