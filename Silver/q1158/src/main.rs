use std::{collections::VecDeque, io};
use io::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let num = buf.split_whitespace()
                                .map(|s|s.trim().parse::<usize>().unwrap())
                                .collect::<Vec<_>>();
    let mut vec : VecDeque<usize> = VecDeque::new();

    for i in 1..=num[0] {
        vec.push_back(i);
    }

    let mut res_vec = Vec::new();
    let mut iter = num[1] - 1;
    while !vec.is_empty() {

        if iter >= vec.len() {
            iter -= vec.len();
        } else { 
            res_vec.push(vec[iter]);
            vec.remove(iter);
            iter += num[1] - 1;
        }
    }

    let mut result = String::new();
    result.push('<');
    for i in 0..res_vec.len() {
        result.push_str(&res_vec[i].to_string());
        if i != res_vec.len()-1 {
            result.push_str(", ");
        }
    } result.push('>');
    write!(out, "{}",result).unwrap();
}