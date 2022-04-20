use std::io::{Write, BufWriter, self};

fn dfs ( graph : Vec<Vec<usize>>, mut visited : Vec<bool>) -> usize {

    let mut tmp = Vec::new(); // Stack
    let mut cnt = 0;
    visited[1] = true;
    tmp.push(1);

    while !tmp.is_empty() {
        let current = tmp.pop().unwrap();
        for i in 0..graph[current].len(){
            let next = graph[current][i];
            if !visited[next] {
                visited[next] = true;
                tmp.push(current);
                tmp.push(next); cnt += 1;
                break;
            }
        }
    } return cnt;
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let k = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let v = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut networks : Vec<Vec<usize>> = vec![vec![];k+1];
    let mut visited : Vec<bool> = vec![false; k+1];
    for _ in 0..v {
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let node_s = iter.next().unwrap().trim().parse::<usize>().unwrap();
        let node_e = iter.next().unwrap().trim().parse::<usize>().unwrap();

        networks[node_s].push(node_e);
        if node_s != node_e { networks[node_e].push(node_s); }
        buf.clear();
    } 
    
    for i in 0..networks.len() {
        networks[i].sort();
    }
    
    write!(out, "{}", dfs(networks, visited)).unwrap();
}
