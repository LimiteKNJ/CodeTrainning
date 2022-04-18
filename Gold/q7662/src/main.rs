use std::io::{Read, Write, self};
use std::collections::BinaryHeap;

/* Double Priority Queue => impl BinaryHeap * 2 */
/* 73083 참고함, Rust에서는 BinaryHeap 메모리 직접 접근이 불가능 */

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
        let process_cnt = buf.trim().parse::<usize>().unwrap();
        buf.clear();

        let mut priority_max : BinaryHeap<isize> = BinaryHeap::new();
        let mut priority_min : BinaryHeap<isize> = BinaryHeap::new();
        let mut pq_tmp_max : BinaryHeap<isize> = BinaryHeap::new();
        let mut pq_tmp_min : BinaryHeap<isize> = BinaryHeap::new();
        let mut cnt = 0;

        for _ in 0..process_cnt {
            stdin.read_line(&mut buf).unwrap();
            let mut iter = buf.split_whitespace();
            let op = iter.next().unwrap();
            let num = iter.next().unwrap().parse::<isize>().unwrap();

            match op {
                "I" => {
                    priority_max.push(num);
                    priority_min.push(num * -1);
                    cnt += 1;
                },
                "D" => {
                    if num == 1 {
                        if cnt > 0 {
                            while !pq_tmp_max.is_empty() && pq_tmp_max.peek() == priority_max.peek() {
                                pq_tmp_max.pop().unwrap();
                                priority_max.pop().unwrap();
                            }
                            let max = priority_max.pop().unwrap();
                            pq_tmp_min.push(max * -1);
                            cnt -= 1;
                        }

                    } else if num == -1 {
                        if cnt > 0 {
                            while !pq_tmp_min.is_empty() && pq_tmp_min.peek() == priority_min.peek() {
                                pq_tmp_min.pop().unwrap();
                                priority_min.pop().unwrap();
                            }
                            let min = priority_min.pop().unwrap() * -1;
                            pq_tmp_max.push(min);
                            cnt -= 1;
                        }
    
                    } else {
                        continue;
                    }
                },

                _ => {

                }
            };

            if cnt == 0 {
                priority_max.clear(); priority_min.clear();
                pq_tmp_max.clear(); pq_tmp_min.clear();
            } buf.clear();
        }
        
        if cnt == 0 {
            write!(out,"EMPTY\n").unwrap();
        } else {
            while !pq_tmp_max.is_empty() && pq_tmp_max.peek() == priority_max.peek() {
                pq_tmp_max.pop().unwrap();
                priority_max.pop().unwrap();
            } let max = priority_max.peek().unwrap() * 1;

            while !pq_tmp_min.is_empty() && pq_tmp_min.peek() == priority_min.peek() {
                pq_tmp_min.pop().unwrap();
                priority_min.pop().unwrap();
            } let min = priority_min.peek().unwrap() * -1;

            if cnt == 1 {
                write!(out,"{} {}\n", max, max).unwrap();
            } else {
                write!(out,"{} {}\n", max, min).unwrap();
            }
        }
    }
}