use std::io::Read;

// --- Day 7: Internet Protocol Version 7 ---
//
// While snooping around the local network of EBHQ, you compile a list of IP
// addresses (they're IPv7, of course; IPv6 is much too limited). You'd like to
// figure out which IPs support TLS (transport-layer snooping).
//
// An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or
// ABBA. An ABBA is any four-character sequence which consists of a pair of two
// different characters followed by the reverse of that pair, such as xyyx or
// abba. However, the IP also must not have an ABBA within any hypernet
// sequences, which are contained by square brackets.
//
// For example:
//
// abba[mnop]qrst supports TLS (abba outside square brackets).
// abcd[bddb]xyyx does not support TLS (bddb is within square brackets, even though xyyx is outside square brackets).
// aaaa[qwer]tyui does not support TLS (aaaa is invalid; the interior characters must be different).
// ioxxoj[asdfgh]zxcvbn supports TLS (oxxo is outside square brackets, even though it's within a larger string).
// How many IPs in your puzzle input support TLS?
//

fn is_abba(s: &[u8]) -> bool {
    for w in s.windows(4) {
        if w[0] != w[1] && w[0] == w[3] && w[1] == w[2] {
            return true;
        }
    }
    false
}

fn dehyper(input: &str) -> (String, String){
    let mut plain = String::new();
    let mut hyper = String::new();

    let mut inspace = false;
    for c in input.chars() {
        match (c, inspace) {
            ('[', true) => {
                inspace = true;
            }
            ('[', false) => {
                // turns out we weren't really in space
            }
            (']', _) => {
                inspace = false;
            }
            (_, false) => plain.push(c),
            (_, true) => hyper.push(c),
        }
    }

    return (plain, hyper)
}

fn main() {
    let mut input = String::new();
    std::fs::File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();
    let mut count = 0;
    for line in input.split('\n') {
	let (plain, hyper) = dehyper(line);
        if is_abba(plain.as_bytes()) && !is_abba(hyper.as_bytes()) {
            count += 1;
            println!("yes {}", line);
        } else {
            println!("no {}", line);
        }

    }
    println!("Final count {}", count);
}
