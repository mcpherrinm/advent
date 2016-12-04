use std::io::prelude::*;
use std::collections::btree_map::BTreeMap;


fn main() {
    let mut input = String::new();
    std::fs::File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();
    let mut sum = 0;
    for line in input.split("\n") {
        let mut count = BTreeMap::new();
        let mut parts = line.split(" ");
        let name = parts.next().unwrap();
        let sector: i32 = parts.next().unwrap().parse().unwrap();
        let checksum = parts.next().unwrap();
        //println!("Name: /{}/ Sector: /{}/ Checksum: /{}/", name, sector, checksum);

        for c in name.chars() {
            *count.entry(c).or_insert(0) += 1;
        }
        count.remove(&'-');
        let mut freqs = count.iter().map(|(b, c)| (10000 - *c, *b)).collect::<std::vec::Vec<(i32, char)>>();
        freqs.sort();
        //println!("freqs for name {}: {:?})", name, freqs);

        freqs.truncate(5);

        let recheck = freqs.iter().map(|&(_, a)| a).collect::<std::string::String>();

        if checksum == recheck {
            //println!("Real: {} ({})", name, recheck);
            sum += sector;
        } else {
            //println!("Not: {} ({})", name, recheck);
        }

        let offset = sector % 26;
        println!("{} {}", name, sector);

        // Decrypting
        for c in name.chars() {
            if c == '-' {
                print!(" ");
                continue;
            }
            let b = c as i32;
            let mut plain = b + offset;
            if plain > ('z' as i32) {
                plain -= 26;
            }
            print!("{}", plain as u8 as char);
        }
        println!(" {}", sector)
    }
    println!("sumsum is {}", sum)

}
