use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    }

    let mut result = true;
    let mut map = HashMap::<u32, u32>::new();

    for num in a {
        *map.entry(num).or_default() += 1;
    }

    for num in b {
        if let Some(&count) = map.get(&num) {
            if 1 <= count {
                map.insert(num, count - 1);
            } else {
                result = false;
            }
        } else {
            result = false;
        }
    }

    println!("{}", if result {"Yes"} else {"No"});
}
