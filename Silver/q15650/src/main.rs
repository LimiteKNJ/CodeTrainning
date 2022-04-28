use std::io;
use std::fmt::Write;

fn combination(vec : &mut Vec<usize>, visited : &mut Vec<bool>, start : usize, n : usize, r : usize, out : &mut String) {

    if r == 0 {
        for i in 0..vec.len() {
            if visited[i] {
                write!(out, "{} ", &vec[i]).unwrap();
            } 
        } write!(out, "\n").unwrap();
        return;
    }

    for i in start..n {
        visited[i] = true;
        combination(vec, visited, i+1, n, r-1, out);
        visited[i] = false;
    }
}

fn main() {

    let stdin = io::stdin();
    let mut buf = String::new();
    let mut out = String::new();

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let r = iter.next().unwrap().trim().parse::<usize>().unwrap();

    let mut vec : Vec<usize> = (1..=n).collect();
    let mut visited = vec![false; n];

    combination(&mut vec, &mut visited, 0 ,n, r, &mut out);
    print!("{}", out);
}