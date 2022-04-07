use std::io;
use io::Write;

fn permutation(vec : &mut Vec<usize>, temp : &mut Vec<usize>, visited : &mut Vec<bool>, n : usize, r : usize) {

    if r == temp.len() {
        print(&temp, temp.len());
        return;
    }

    for i in 0..n {
        if !visited[i] {
            visited[i] = true;
            temp[r] = vec[i];
            permutation(vec, temp, visited, n, r+1);
            visited[i] = false;
        }
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
    let mut vec : Vec<usize> = buf.split_whitespace()
                                .map(|s|s.trim().parse::<usize>().unwrap())
                                .collect::<Vec<_>>();
    let mut temp : Vec<usize> = vec![0; r];
    let mut visited : Vec<bool> = vec![false; n];
    vec.sort();

    permutation(&mut vec, &mut temp, &mut visited, n, 0);
}