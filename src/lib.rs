fn day1_part1(path: &str) -> u32 {
    let content = std::fs::read_to_string(path).unwrap_or_default();

    let mut list: Vec<u32> = content
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .filter_map(|food| food.parse::<u32>().ok())
                .sum()
        })
        .collect();

    list.sort();
    list.reverse();
    list[0]
}

fn day1_part2(path: &str) -> u32 {
    let content = std::fs::read_to_string(path).unwrap_or_default();

    let mut list: Vec<u32> = content
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .filter_map(|food| food.parse::<u32>().ok())
                .sum()
        })
        .collect();

    list.sort();
    list.reverse();
    let top_three = list[0..3].to_vec();

    top_three.iter().sum()
}


#[test]
fn is_corrent_by_example_part1() {
    let res = day1_part1("./data/1.example.txt");
    assert_eq!(res, 24000)
}

#[test]
fn is_corrent_by_example_part2() {
    let res = day1_part2("./data/1.example.txt");
    assert_eq!(res, 45000)
}

#[test]
fn is_corrent_part1() {
    let res = day1_part1("./data/1.txt");
    assert_eq!(res, 67633)
}

#[test]
fn is_corrent_part2() {
    let res = day1_part2("./data/1.txt");
    assert_eq!(res, 199628)
}
