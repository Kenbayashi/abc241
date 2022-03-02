use std::io;
use std::io::Read;

fn main() {
    let q = read();

    let mut a = Vec::<i32>::new();

    for _ in 0..q {
        let query = read();

        match &query {
            1 => one(&mut a, read()),
            2 => two(a.clone(), read(), read()),
            3 => three(a.clone(), read(), read()),
            _ => (),
        }

        eprintln!("{:?}", a);
    }
}

fn read() -> i32 {
    io::stdin().bytes()
               .map(|c| c.unwrap() as char)
               .skip_while(|&c| c.is_whitespace())
               .take_while(|&c| !c.is_whitespace())
               .collect::<String>()
               .parse::<i32>()
               .ok()
               .unwrap()
}

fn one(a: &mut Vec<i32>, x: i32) {
    a.push(x);
}

fn two(a: Vec<i32>, x: i32, k: i32) {
    let mut selected = a.into_iter()
                        .filter(|&num| num <= x)
                        .collect::<Vec<i32>>();

    selected.sort_by(|a, b| b.cmp(a));

    let result = match selected.get((k - 1) as usize) {
        Some(&num) => num,
        None       => -1,
    };

    println!("{}", result);
}

fn three(a: Vec<i32>, x: i32, k: i32) {
    let mut selected = a.into_iter()
                        .filter(|&num| x <= num)
                        .collect::<Vec<i32>>();

    selected.sort_by(|a, b| a.cmp(b));

    let result = match selected.get((k - 1) as usize) {
        Some(&num) => num,
        None       => -1,
    };

    println!("{}", result);
}
