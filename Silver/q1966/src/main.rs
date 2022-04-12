use std::io;
use io::Write;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

/* Priority Queue => impl BinaryHeap */

fn main() {
    
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let case = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    for _ in 0..case {
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let print_cnt = iter.next().unwrap().trim().parse::<usize>().unwrap();
        let print_known = iter.next().unwrap().trim().parse::<usize>().unwrap();
        buf.clear();

        stdin.read_line(&mut buf).unwrap();
        let print_priority_input = buf.split_whitespace().
            map(|s|s.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        buf.clear();

        let mut print_priority = VecDeque::new();
        let mut print_priority_pq = BinaryHeap::new();
        let mut idx = 0;
        for i in print_priority_input {
            print_priority.push_back((idx, i));
            print_priority_pq.push(i);
            idx += 1;
        }

        idx = 1;
        while !print_priority.is_empty() {
            if print_priority.front().unwrap().1 == *print_priority_pq.peek().unwrap() {
                if print_priority.front().unwrap().0 == print_known {
                    writeln!(out, "{}", idx).unwrap();
                    break;
                }
                print_priority_pq.pop();
                idx += 1;

            } else {
                print_priority.push_back(*print_priority.front().unwrap());
            } print_priority.pop_front();
        }
    }
}