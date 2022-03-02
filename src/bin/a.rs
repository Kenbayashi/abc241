fn main() {
    proconio::input! {
        a: [usize; 10],
    }

    let mut next = 0;
    for _ in 0..3 {
        next = a[next];
    }

    println!("{}", next);
}
