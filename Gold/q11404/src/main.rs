use std::io::{Write, BufWriter, self};

fn floyd (mut graph : Vec<Vec<usize>>) -> Vec<Vec<usize>> {

    for m in 1..graph.len(){
        for s in 1..graph.len(){
            for e in 1..graph.len(){
                if s == e {
                    graph[s][e] = 0;
                } else if s == m {
                    graph[s][m] = 0; 
                } else if m == e {
                    graph[m][e] = 0;
                } else {
                    if graph[s][e] > graph[s][m] + graph[m][e] {
                        graph[s][e] = graph[s][m] + graph[m][e];
                    }
                }
            }
        }
    }
    return graph;
}

fn main() {
    static MAX_DATA: usize = 10000000;
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap(); 
    let k = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap(); 
    let v_cnt = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut graph = vec![vec![MAX_DATA; k+1];k+1];
    for _ in 0..v_cnt {
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let s = iter.next().unwrap().parse::<usize>().unwrap();
        let e = iter.next().unwrap().parse::<usize>().unwrap();
        let v = iter.next().unwrap().parse::<usize>().unwrap();
        if v < graph[s][e] {
            graph[s][e] = v;
        } buf.clear();
    }

    let result = floyd(graph);
    let mut text = String::new();
    for i in 1..result.len() {
        for j in 1..result.len() {
            if result[i][j] == MAX_DATA {
                text.push('0');
            } else {
                text.push_str(&result[i][j].to_string());
            } text.push(' ');
        } text.push('\n');
    } write!(out,"{}",text).unwrap();
}
