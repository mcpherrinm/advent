use std::io::Read;
fn main() {
    let mut current = 5;
    let mut input = String::new();
    std::fs::File::open("input.txt").unwrap().read_to_string(&mut input);
    for b in input.chars() {
        match b {
            'U' => {
                if current > 3 {
                    current -= 3
                }
            }
            'L' => {
                if current != 1 && current != 4 && current != 7 {
                    current -= 1
                }
            }
            'R' => {
                if current != 3 && current != 6 && current != 9 {
                    current += 1
                }
            }
            'D' => {
                if current < 7 {
                    current += 3
                }
            }
            '\n' => {
                print!("{}", current);
            }
            _ => {}
        }
    }
}
