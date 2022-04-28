use std::io;
use std::fmt::Write;
use std::collections::VecDeque;

fn main() {  
    let stdin = io::stdin();

    let mut out = String::new();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let size = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut stack = Vec::new();
    let mut ques = VecDeque::new();
    for _ in 0..size {
        stdin.read_line(&mut buf).unwrap();
        ques.push_back(buf.trim().parse::<usize>().unwrap());
        buf.clear();
    }

    let mut i = 1; let mut suc = true;
    while !ques.is_empty() {
        if !stack.is_empty() && stack.last().unwrap() == ques.front().unwrap(){
            stack.pop().unwrap(); ques.pop_front().unwrap();
            write!(out, "-\n").unwrap();
        } else if i <= match ques.front(){Some(&n) => n , None => 0}{
            while i <= match ques.front(){Some(&n) => n , None => 0} {
                stack.push(i); i += 1;
                write!(out, "+\n").unwrap();
            }
        } else {
            suc = false; break;
        }
    }

    if suc {
        print!("{}", out);
    } else {
        print!("NO");
    }
}