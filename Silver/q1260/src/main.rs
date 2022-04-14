/* BFS and DFS */

use std::io::{Read, Write, BufWriter, self};
use std::collections::VecDeque;

fn print_n (n : usize){
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    write!(out, "{} ", n).unwrap();
}

fn print_ln() {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    write!(out, "\n").unwrap();
}

fn dfs(graph : Vec<Vec<usize>>, mut visited : Vec<bool>, start_node : usize) {
    
    let mut tmp = Vec::new();
    visited[start_node] = true;
    tmp.push(start_node);
    print_n(start_node);

    while !tmp.is_empty() {
        let current = tmp.pop().unwrap();
        for i in 0..graph[current].len() {
            let next = graph[current][i];
            if !visited[next] {
                print_n(next);
                visited[next] = true;
                tmp.push(current);
                tmp.push(next);
                break;
            }
        }
    }
}

fn bfs(graph : Vec<Vec<usize>>, mut visited : Vec<bool>, start_node : usize) {
    
    let mut tmp = VecDeque::new();
    visited[start_node] = true;
    tmp.push_back(start_node);

    while !tmp.is_empty() {
        let current = tmp.pop_front().unwrap();
        print_n(current);

        for i in 0..graph[current].len() {
            let next = graph[current][i];
            if !visited[next] {
                tmp.push_back(next);
                visited[next] = true;
            }
        }
    }
} 

fn main() {   
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let v_num = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let u_num = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let start = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    let visited : Vec<bool> = vec![false; v_num+1];
    let mut graph : Vec<Vec<usize>> = vec![vec![]; v_num+1];
    for _ in 0..u_num {
        stdin.read_line(&mut buf).unwrap();
        let mut iter2 = buf.split_whitespace();
        let s = iter2.next().unwrap().trim().parse::<usize>().unwrap();
        let e = iter2.next().unwrap().trim().parse::<usize>().unwrap();

        graph[s].push(e);
        if s != e { graph[e].push(s); }
        buf.clear();
    }

    for i in 0..graph.len(){
        graph[i].sort();
    }

    dfs(graph.clone(), visited.clone(), start);
    print_ln();
    bfs(graph.clone(), visited.clone(), start);
}
