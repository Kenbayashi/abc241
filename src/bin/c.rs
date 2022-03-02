use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut result = false;
    let direction: Vec<(isize, isize)> = vec![(1, 0), (1, -1), (0, 1), (-1, -1)];

    for x in 0..(n as isize) {
        for y in 0..(n as isize) {
            for &(dx, dy) in &direction {
                result = result || check(x, y, dx, dy, &s);
            }
        }
    }

    println!("{}", if result {"Yes"} else {"No"});
}

fn check(x: isize, y: isize, dx: isize, dy: isize, s: &Vec<Vec<char>>) -> bool {
    let mut count = 0;

    for i in 0..6 {
        if let Some(sx) = s.get((x + dx * i) as usize) {
            if let Some(&c) = sx.get((y + dy * i) as usize) {
                if c == '#' {
                    count += 1;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    4 <= count
}
