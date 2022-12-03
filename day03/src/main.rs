fn main() {
    let lines = include_str!("../input.txt")
            .lines();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut result1 = 0;
    let mut result2 = 0;

    for (index, line1) in lines.clone().enumerate() {
        let items = line1.chars().count() / 2;
        let (a, b) = line1.split_at(items);

        for c in a.chars() {
            if b.contains(c) {
                result1 += alphabet.iter().position(|&r| r == c).unwrap() + 1;
                break;
            }
        }

        if index % 3 == 0 {
            let line2 = lines.clone().nth(index + 1).unwrap();
            let line3 = lines.clone().nth(index + 2).unwrap();

            for c in line1.chars() {
                if line2.contains(c) && line3.contains(c) {
                    result2 += alphabet.iter().position(|&r| r == c).unwrap() + 1;
                    break;
                }
            }
        }
    }

    println!("Part One: {}", result1);
    println!("Part Two: {}", result2);
}
