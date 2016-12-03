#![feature(step_by)]
mod input;
use input::INPUT;

fn main() {
    let mut c = 0;
    for i in (0..INPUT.len()).step_by(3) {
        for j in 0..3 {
            let mut v = std::vec::Vec::new();
            v.push(INPUT[i][j]);
            v.push(INPUT[i + 1][j]);
            v.push(INPUT[i + 2][j]);
            v.sort();
            if v[0] + v[1] <= v[2] {
                println!("{} + {} < {}", v[0], v[1], v[2]);
            } else {
                println!("!{} + {} < {}", v[0], v[1], v[2]);
                c += 1;
            }
        }
    }
    println!("Count: {}", c);
}
