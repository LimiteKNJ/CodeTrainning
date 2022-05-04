use std::io;
use std::fmt::Write;

fn get_pi(pattern : Vec<char>) -> Vec<usize> {
    
    let mut pi = Vec::new(); pi.resize(pattern.len(), 0);
    let mut j = 0;
    for i in 1..pi.len() {
        while j > 0 && pattern[i] != pattern[j]{
            j = pi[j-1];

        } if pattern[i] == pattern[j] {
            j += 1; pi[i] = j;
        }
    }
    return pi;
}

fn kmp(input : Vec<char>, find : Vec<char>) -> Vec<usize> {

    let mut result = Vec::new();
    let pi = get_pi(find.clone());
    let mut j = 0;

    for i in 0..input.len(){
        while j > 0 && input[i] != find[j] {
            j = pi[j-1];

        } if input[i] == find[j] {
            if j == find.len() - 1 {
                result.push(i - (find.len() - 1));
                j = pi[j];

            } else {
                j += 1;
            }
        }
    }

    return result;
}

fn main() {

    let stdin = io::stdin();
    let mut out = String::new();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut input = buf.chars().collect::<Vec<_>>();
    buf.clear();
    if input.last().unwrap() == &'\n' { input.pop().unwrap(); }
    if input.last().unwrap() == &'\r' { input.pop().unwrap(); }
    
    stdin.read_line(&mut buf).unwrap();
    let mut find = buf.chars().collect::<Vec<_>>();
    buf.clear();
    if find.last().unwrap() == &'\n' { find.pop().unwrap(); }
    if find.last().unwrap() == &'\r' { find.pop().unwrap(); }

    let result = kmp(input, find);
    write!(out, "{}\n", result.len()).unwrap();
    for i in result {
        write!(out, "{} ", i+1).unwrap();
    } print!("{}", out);
}