use std::io::Read;
const A: i32 = 10;
const B: i32 = 11;
const C: i32 = 12;
const D: i32 = 13;
const X: i32 = 0;

const GRID: [[i32; 7]; 7] = [[X, X, X, X, X, X, X],
                             [X, X, X, 1, X, X, X],
                             [X, X, 2, 3, 4, X, X],
                             [X, 5, 6, 7, 8, 9, X],
                             [X, X, A, B, C, X, X],
                             [X, X, X, D, X, X, X],
                             [X, X, X, X, X, X, X]];

fn i(x: i32, y: i32) -> i32 {
    GRID[y as usize][x as usize]
}

fn main() {
    let mut input = String::new();
    //input = "ULL\nRRDDD\nLURDL\nUUUUD\n".into();
    std::fs::File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();

    let (mut x, mut y) = (1, 3);
    println!("Starting at {:?} = {}", (x, y), i(x, y));

    for b in input.chars() {
        match b {
            'U' if i(x, y - 1) != X => {
                y -= 1;
            }
            'D' if i(x, y + 1) != X => {
                y += 1;
            }
            'L' if i(x - 1, y) != X => {
                x -= 1;
            }
            'R' if i(x + 1, y) != X => {
                x += 1;
            }
            '\n' => {
                let v = i(x, y);
                if v < 10 {
                    print!("{}", v);
                } else {
                    print!("{}", b"ABCD"[(v - 10) as usize] as char);
                }
            }
            _ => {}
        }
    }
}
