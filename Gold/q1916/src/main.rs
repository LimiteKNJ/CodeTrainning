/* Dijkstra Algorithm Practice */
// Priority Queue + BinaryHeap Tree

use std::io::{Write, BufWriter, self};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) {

    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {

        if position == goal { print_cost(Some(cost)); }
        if cost > dist[position] { continue; }

        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
}

fn print_cost (n :Option<usize>) {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    match n {
        Some(i) => write!(out, "{}", i).unwrap(),
        None => write!(out, "INF").unwrap()
    }
    
}

fn main() {

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let v_cnt = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    
    stdin.read_line(&mut buf).unwrap();
    let e_cnt = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut graph : Vec<Vec<Edge>> = vec![vec![]; v_cnt+1];
    let mut nodes : Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..e_cnt {
        stdin.read_line(&mut buf).unwrap();
        let mut iter2 = buf.split_whitespace();
        let u = iter2.next().unwrap().trim().parse::<usize>().unwrap();
        let v = iter2.next().unwrap().trim().parse::<usize>().unwrap();
        let e = iter2.next().unwrap().trim().parse::<usize>().unwrap();
        buf.clear();

        nodes.push((u, v, e));
    }

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let start_k = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let end_k = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    for n in nodes {
        graph[n.0].push(Edge { node: n.1, cost: n.2 } );
    }

    shortest_path(&graph, start_k, end_k);
}