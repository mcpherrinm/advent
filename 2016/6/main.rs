use std::io::Read;

fn main() {
    let mut freqs = Vec::new();
    for _ in 0..8 {
        freqs.push(std::collections::btree_map::BTreeMap::new());
    }
    let mut input = String::new();
    std::fs::File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();
    for line in input.split('\n') {
        for (i, b) in line.chars().enumerate() {
            *freqs[i].entry(b).or_insert(0) += 1;
        }
    }
    for i in 0..8 {
        let mut thismax = 0;
        let mut thismin = 99999;
        let mut maxC = '!';
        let mut minC = '!';
        for (&k, &v) in freqs[i].iter() {
            if v < thismin {
                minC = k;
                thismin = v;
            }
            if v > thismax {
                maxC = k;
                thismax = v;
            }
        }
        //print!("{}", maxC);
        print!("{}", minC);
    }
}