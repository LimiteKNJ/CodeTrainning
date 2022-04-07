use std::io;
use io::Write;

fn permutation(vec : &mut Vec<usize>, temp : &mut Vec<usize>, n : usize, r : usize) {

    if r == temp.len() {
        let stdout = std::io::stdout();
        let mut out = io::BufWriter::new(stdout.lock());
        let mut result = String::new();

        for i in 0..temp.len() {
            result.push_str(&(temp[i].to_string() + " "));
        } result.push_str("\n");
        write!(out, "{}", result).unwrap();
        return;
    }

    for i in 0..n {
            temp[r] = vec[i];
            permutation(vec, temp, n, r+1);
    }
}

fn main() {

    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let r = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    let mut vec : Vec<usize> = (1..=n).collect();
    let mut temp : Vec<usize> = vec![0; r];

    permutation(&mut vec, &mut temp, n, 0);
}