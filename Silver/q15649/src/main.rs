use std::io;
use std::fmt::Write;

fn permutation(vec : &mut Vec<usize>, temp : &mut Vec<usize>, visited : &mut Vec<bool>, n : usize, r : usize, out : &mut String) {

    if r == temp.len() {
        for i in 0..temp.len() {
            write!(out, "{} ", temp[i]).unwrap();
        } write!(out,"\n").unwrap();
        return;
    }

    for i in 0..n {
        if visited[i] { continue; }
        visited[i] = true;
        temp[r] = vec[i];
        permutation(vec, temp, visited, n, r+1, out);
        visited[i] = false;
    }
}
fn main() {

    let stdin = io::stdin();
    let mut buf = String::new();
    let mut buf_out = String::new();

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let r = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    let mut vec : Vec<usize> = (1..=n).collect();
    let mut visited : Vec<bool> = vec![false; n];
    let mut temp : Vec<usize> = vec![0; r];

    permutation(&mut vec, &mut temp, &mut visited, n, 0, &mut buf_out);
    print!("{}", buf_out);
}