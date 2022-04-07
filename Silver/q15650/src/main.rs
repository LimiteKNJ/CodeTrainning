use std::io;
use io::Write;

fn combination(vec : &mut Vec<usize>, visited : &mut Vec<bool>, start : usize, n : usize, r : usize) {

    if r == 0 {
        print(vec, visited, n);
        return;
    }

    for i in start..n {
        visited[i] = true;
        combination(vec, visited, i+1, n, r-1);
        visited[i] = false;
    }

}

fn print(vec : &Vec<usize>, visited : &Vec<bool>, n : usize) {

    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for i in 0..n {
        if visited[i] {
            write!(out, "{} ", &vec[i]).unwrap();
        } 
    } writeln!(out).unwrap();
}

fn main() {

    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let mut n = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let mut r = iter.next().unwrap().trim().parse::<usize>().unwrap();

    let mut vec : Vec<usize> = (1..=n).collect();
    let mut visited = vec![false; n];

    combination(&mut vec, &mut visited, 0 ,n, r);
}