use std::io::{Read, Write, BufWriter, self};
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {

    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] { continue; }

        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }   
    }

    return dist;
}

fn main() {

    let stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let v_cnt = iter.next().unwrap().trim().parse::<usize>().unwrap();
    let e_cnt = iter.next().unwrap().trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let start_k = buf.trim().parse::<usize>().unwrap();
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

    for n in nodes {
        graph[n.0].push(Edge { node: n.1, cost: n.2 } );
    }

    let result = shortest_path(&graph, start_k);
    for i in 1..=v_cnt {
        if result[i] == usize::MAX {
            write!(out, "INF\n").unwrap();
        } else {
            write!(out, "{}\n", result[i]).unwrap();
        }
    }
}