use std::io::{Write, BufWriter, self};

fn main() {
     
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
//  let mut str = buf.trim().to_string();
    let str = buf.trim().chars().collect::<Vec<_>>();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
//  let bomb = buf.trim().to_string();
    let bomb = buf.trim().chars().collect::<Vec<_>>();
    buf.clear();

    let mut stack = Vec::new();
    for c in str {
        stack.push(c);
        if stack.len() >= bomb.len() {
            if c == bomb[bomb.len()-1] && stack[stack.len()-bomb.len()..] == bomb {
                for _ in 0..bomb.len() { stack.pop().unwrap(); }
            }
        }
    } if stack.is_empty() {
        write!(out,"FRULA").unwrap();
    } else {
        let mut result = String::new();
        for n in stack {
            result.push(n);
        } write!(out, "{}", result).unwrap();
    }

    /* Read String to EOL
    while str.contains(&bomb) {
        let tmp = str.split(&bomb).collect::<Vec<_>>();
        str = tmp.concat();
    } 
    
    if str.is_empty() {
        write!(out,"FRULA").unwrap();
    } else {
        write!(out, "{}", str).unwrap();
    }
     */
}