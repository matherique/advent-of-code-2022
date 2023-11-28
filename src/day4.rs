use std::{fs, io::Result};

fn main() -> Result<()> {
    // let content = fs::read_to_string("./data/4.example.txt")?;
    let content = fs::read_to_string("./data/4.txt")?;

    let total: Vec<usize> = content
        .trim_end()
        .split("\n")
        .map(|line| {
            let pairs: Vec<&str> = line.split(",").collect();

            let mut sections: Vec<String> = Vec::new();

            // print!("{line}");
            for part in pairs {
                let (start, end) = if let Some((s, e)) = part.split_once("-") {
                    (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap())
                } else {
                    (0, 0)
                };

                // print!(" start {} end {}", start, end);

                let mut part1 = "".to_string();
                if start != end {
                    for i in start..end+1 {
                        part1 += &i.to_string();
                    }
                } else {
                    part1 += &start.to_string();
                }

                sections.push(part1);
            }
            // print!("\n");

            let mut response = 0;
            if let Some(first) = sections.get(0) {
                if let Some(second) = sections.get(1) {
                    if first.contains(second) || second.contains(first) {
                        response = 1;
                    }
                }
            }

            return response;
        })
        .collect();

    let sum: usize = total.iter().sum();
    println!("{:?}", sum);

    return Ok(());
}
