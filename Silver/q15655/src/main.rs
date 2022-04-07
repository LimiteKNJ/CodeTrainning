use std::io;
use io::Write;

fn combination(vec : &mut Vec<usize>, temp : &mut Vec<usize>, start : usize, n : usize, r : usize) {

    if r == temp.len() {
        print(&temp, temp.len());
        return;
    }

    for i in start..n {
        temp[r] = vec[i]; 
        combination(vec, temp, i+1, n, r+1);
    }

}

fn print(vec : &Vec<usize>, n : usize) {

    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut result = String::new();

    for i in 0..n {
        result.push_str(&(vec[i].to_string() + " "));
    } result.push_str("\n");
    write!(out, "{}", result).unwrap();
}

fn main() {

    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let r = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let mut vec = buf.split_whitespace()
                                .map(|s|s.trim().parse::<usize>().unwrap())
                                .collect::<Vec<_>>();
    let mut temp : Vec<usize> = vec![0; r];
    vec.sort();

    combination(&mut vec, &mut temp, 0 ,n, 0);
}