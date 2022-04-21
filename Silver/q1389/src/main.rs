/* BFS and DFS */

use std::io::{Write, BufWriter, self};

fn floyd_warshall(mut graph : Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    for m in 1..graph.len(){
        for s in 1..graph.len(){
            for e in 1..graph.len(){
                if graph[s][e] > graph[s][m] + graph[m][e] {
                    graph[s][e] = graph[s][m] + graph[m][e];
                }
            }
        }
    } return graph;
} 

fn main() {   
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let user = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let rel = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    let mut graph : Vec<Vec<usize>> = vec![vec![10000000; user+1]; user+1];
    for _ in 0..rel {
        stdin.read_line(&mut buf).unwrap();
        let mut iter2 = buf.split_whitespace();
        let s = iter2.next().unwrap().trim().parse::<usize>().unwrap();
        let e = iter2.next().unwrap().trim().parse::<usize>().unwrap();

        graph[s][e] = 1;
        if s != e { graph[e][s] = 1; }
        buf.clear();
    }

    let result = floyd_warshall(graph);
    let mut sumList = Vec::new();
    for i in 1..result.len(){
        let mut sum = 0;
        for j in 1..result.len(){
            if i != j {
                sum += result[i][j];
            }
        } sumList.push(sum);
    }
    let mut min = 10000000;
    let mut idx = 0;
    for i in 0..sumList.len() {
        if sumList[i] < min {
            min = sumList[i]; idx = i+1; 
        }
    } write!(out, "{}", idx).unwrap();
}