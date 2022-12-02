fn action_score(a: char) -> i32 {
    return match a {
        'X'=> 1,
        'Y'=> 2,
        'Z'=> 3,
        _ => 0,
    }
}

fn outcome_score(a:char, b: char) -> i32 {
    return if b == get_action_by_result(a, 'Y') {
        3
    } else if b == get_action_by_result(a, 'Z'){
        6
    } else {
        0
    }
}

fn get_action_by_result(a: char, result: char) -> char {
    return match result {
        'X'=> {
            match a {
                'A'=> 'Z',
                'B'=> 'X',
                'C'=> 'Y',
                _ => '-',
            }
        },
        'Y'=> {
            match a {
                'A'=> 'X',
                'B'=> 'Y',
                'C'=> 'Z',
                _ => '-',
            }
        },
        'Z'=> {
            match a {
                'A'=> 'Y',
                'B'=> 'Z',
                'C'=> 'X',
                _ => '-',
            }
        },
        _ => '-',
    }
}

fn main() {
    let items = include_str!("../input.txt")
            .lines();
    let mut score1 = 0;
    let mut score2 = 0;


    for item in items {
        let a = item.chars().nth(0).unwrap();
        let b = item.chars().nth(2).unwrap();

        score1 += action_score(b)+ outcome_score(a, b);
        
        let action_task_2 = get_action_by_result(a, b);
        score2 += action_score(action_task_2) + outcome_score(a, action_task_2);
    }

    println!("Part One: {}", score1);
    println!("Part Two: {}", score2);
}
