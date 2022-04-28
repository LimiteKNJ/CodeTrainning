use std::io;
use std::fmt::Write;

fn combination(vec : &mut Vec<usize>, temp : &mut Vec<usize>, start : usize, n : usize, r : usize, out : &mut String) {

    if r == temp.len() {
        for i in 0..temp.len() {
            write!(out, "{} ", temp[i]).unwrap();
        } write!(out, "\n").unwrap();
        return;
    }

    for i in start..n {
        temp[r] = vec[i]; 
        combination(vec, temp, i, n, r+1, out);
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
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let mut vec = buf.split_whitespace()
                                .map(|s|s.trim().parse::<usize>().unwrap())
                                .collect::<Vec<_>>();
    let mut temp : Vec<usize> = vec![0; r];
    vec.sort();

    combination(&mut vec, &mut temp, 0 ,n, 0, &mut out);
    print!("{}",out);
}