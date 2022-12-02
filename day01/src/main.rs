

fn main() {
    let items = include_str!("../input.txt")
            .lines();
    let mut elfes: Vec<i32> = vec![0];

    for item in items {
        if item == "" {
            elfes.push(0);
        } else {
            let i = elfes.len() - 1;
            elfes[i] += item.parse::<i32>().unwrap();
        }
    }
    elfes.sort();
    elfes.reverse();


    let result1 = elfes[0];
    let result2 = elfes[0] + elfes[1] + elfes[2];

    println!("Part One: {}", result1);
    println!("Part Two: {}", result2);
}
