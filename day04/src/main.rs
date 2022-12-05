fn main() {
    let lines = include_str!("../input.txt")
            .lines();
    let mut result1 = 0;
    let mut result2 = 0;

    for line in lines {
        let pair: Vec<&str> = line.split(',').collect();
        let a: Vec<i16> = pair[0].split('-').map(|n| n.parse().unwrap()).collect();
        let b: Vec<i16> = pair[1].split('-').map(|n| n.parse().unwrap()).collect();
        let range_a: Vec<i16> = (a[0]..=a[1]).collect();
        let range_b: Vec<i16> = (b[0]..=b[1]).collect();

        if range_a.iter().all(|n| range_b.contains(n)) || range_b.iter().all(|n| range_a.contains(n)) {
            result1 += 1;
        }
        
        if range_a.iter().any(|n| range_b.contains(n)) {
            result2 += 1;
        } 
    }
    
    println!("Part One: {}", result1);
    println!("Part Two: {}", result2);
}