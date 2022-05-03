use std::io::{self, Write, BufWriter};
use std::collections::VecDeque;

fn bfs(start : usize, arrive : usize) -> usize {
    
    let mut tmp = VecDeque::new();
    let mut graph = Vec::new(); graph.resize(100001, usize::MAX);
    let mut visited = Vec::new(); visited.resize(100001, false);
    visited[start] = true; graph[start] = 0;
    tmp.push_back(start);

    let mut ans = 0;
    while !tmp.is_empty() {
        let current = tmp.pop_front().unwrap();
        let act : Vec<isize> = vec![2, -1 , 1];

        if current == arrive {
            ans = graph[current];
            break;
        }

        for i in act {
            if current < 1 && i == -1 || current + 1 > 100000 && i == 1
            || current * 2 > 100000 && i == 2 {
                continue;
            } else {
                if i == 1 || i == -1 {
                    let next = (current as isize + i) as usize;
                    if !visited[next] || graph[next] > graph[current] + 1 {
                        tmp.push_back(next);
                        visited[next] = true;
                        graph[next] = graph[current] + 1;
                    }
                } else { 
                    let next = (current as isize * i) as usize;
                    if !visited[next] || graph[next] > graph[current] {
                        tmp.push_front(next);
                        visited[next] = true;
                        graph[next] = graph[current];
                    }
                }
            }
        }
    }

    return ans;
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

    write!(out, "{}", bfs(start, arr)).unwrap();
}