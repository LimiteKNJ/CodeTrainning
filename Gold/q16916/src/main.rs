use std::io::{Write, BufWriter, self};

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
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let input = buf.trim().chars().collect::<Vec<_>>();
    buf.clear();
    
    stdin.read_line(&mut buf).unwrap();
    let find = buf.trim().chars().collect::<Vec<_>>();
    buf.clear();

    let result = kmp(input, find);
    if result.len() > 0 {
        write!(out, "1").unwrap();
    } else {
        write!(out, "0").unwrap();
    }
}