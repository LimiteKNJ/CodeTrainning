use std::io;
use io::{Write};

fn binary_sreach_for_lower_bound(vec : &Vec<i128>, target : i128) -> usize {

    let mut left = 0; let mut right = vec.len();
    
    while left < right {
        let mid = left + (right-left) / 2;
        
        if target <= vec[mid] {
            right = mid;
        } else {
            left = mid+1;
        }
    }
    return left;
}

fn binary_sreach_for_upper_bound(vec : &Vec<i128>, target : i128) -> usize {

    let mut left = 0; let mut right = vec.len();
    
    while left < right {
        let mid = left + (right-left) / 2;

        if target >= vec[mid] {
            left = mid+1;
        } else {
            right = mid;
        }
    }
    return left;
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let card_cnt = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let mut card_num = buf.split_whitespace()
                                .map(|s|s.trim().parse::<i128>().unwrap())
                                .collect::<Vec<_>>();
    card_num.sort();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let num_input = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let ans = buf.split_whitespace()
                            .map(|s|s.trim().parse::<i128>().unwrap())
                            .collect::<Vec<_>>();
    buf.clear();

    let mut result = String::new();
    for i in ans {
        let upper = binary_sreach_for_upper_bound(&card_num, i);
        let lower = binary_sreach_for_lower_bound(&card_num, i);
        let count = upper - lower;

        result.push_str(&count.to_string());
        result.push_str(" ");
    }  write!(out, "{}", result).unwrap();
}