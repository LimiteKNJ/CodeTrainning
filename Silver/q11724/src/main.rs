use std::io::{Write, BufWriter, self};

fn dfs ( graph : &Vec<Vec<usize>>, visited : &mut Vec<bool>, start : usize) {

    let mut tmp = Vec::new(); // Stack
    
    visited[start] = true;
    tmp.push(start);

    while !tmp.is_empty() {
        let current = tmp.pop().unwrap();
        for i in 0..graph[current].len(){
            let next = graph[current][i];
            if !visited[next] {
                visited[next] = true;
                tmp.push(current);
                tmp.push(next);
                break;
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
    let k = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let v = iter.next().unwrap().trim().parse::<usize>().unwrap();  
    buf.clear();

    let mut node : Vec<Vec<usize>> = vec![vec![];k+1];
    let mut visited : Vec<bool> = vec![false; k+1];
    for _ in 0..v {
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let node_s = iter.next().unwrap().trim().parse::<usize>().unwrap();
        let node_e = iter.next().unwrap().trim().parse::<usize>().unwrap();

        node[node_s].push(node_e);
        if node_s != node_e { node[node_e].push(node_s); }
        buf.clear();
    } 
    
    for i in 0..node.len() {
        node[i].sort();
    }

    let mut cnt = 0;
    for i in 1..=k {
        if visited[i] { continue; }
        dfs(&node, &mut visited, i);
        cnt += 1;
    } 
    write!(out, "{}", cnt).unwrap();
}