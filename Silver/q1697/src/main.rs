use std::io::{self, Write, BufWriter};
use std::collections::VecDeque;

fn bfs(start : usize, arrive : usize) -> (usize, usize) {
    
    let mut visited = Vec::new(); visited.resize(100001, false);
    let mut tmp = VecDeque::new();
    tmp.push_back((start, 0));

    let mut ans = 0;
    let mut cnt = 0;

    while !tmp.is_empty() {
        let cur = tmp.pop_front().unwrap();
        let current = cur.0; let sec = cur.1;
        let act : [isize; 3] = [-1, 1, current as isize];
        visited[current] = true;

        if ans != 0 && ans < sec {
            break;
        }

        if ans == sec && current == arrive { 
            cnt += 1; continue;
        }

        if ans == 0 && current == arrive { 
            ans = sec;
            cnt += 1; continue;
        }
        
        for i in act {
            let next;
            if current < 1 && i == -1 || current + 1 > 100000 && i == 1
            || current * 2 > 100000 && i == current as isize {
                continue;
            } else {
                next = (current as isize + i) as usize;
                if !visited[next] {
                    tmp.push_back((next, sec+1));
                }
            }
        }
    }
    return (ans, cnt);
} 

fn main() {

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let start = iter.next().unwrap().parse::<usize>().unwrap();
    let arr = iter.next().unwrap().parse::<usize>().unwrap();
    buf.clear();

    write!(out, "{}", bfs(start, arr).0).unwrap();
}