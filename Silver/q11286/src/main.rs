/* n^2logn Hashmap + Heap / Sorting

use std::io::{Write, self, BufWriter};
use std::collections::HashMap;
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let size = buf.trim().parse::<isize>().unwrap();
    buf.clear();

    let mut map : HashMap<isize, Vec<isize>> = HashMap::new();
    let mut heap = BinaryHeap::new();
    for _ in 0..size {
        stdin.read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<isize>().unwrap();

        if num == 0 {
            let pop = heap.pop();
            let mut result = 0;
            match pop {
                Some(n) => {
                    for i in map.keys() { // O(n^2logn + n) << 640ms
                        if i == &n {
                            let mut tmp : Vec<isize> = map.get(i).unwrap().to_vec();
                            tmp.sort_by(|a,b| b.cmp(&a)); result = tmp.pop().unwrap();
                            map.remove(&n); map.insert(n, tmp);
                            break;
                        }
                    } write!(out,"{}\n", result).unwrap();
                }
                None => write!(out,"0\n").unwrap()
            } 
        } else {
            map.entry(num.abs() * -1).or_default().push(num);
            heap.push(num.abs() * -1);
        }
        buf.clear();
    }
}

*/

use std::io::{Write, self, BufWriter};
use std::collections::BinaryHeap;

fn main() { // 24ms by Heap<tuple>
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let size = buf.trim().parse::<isize>().unwrap();
    buf.clear();

    let mut heap = BinaryHeap::new();
    for _ in 0..size {
        stdin.read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<isize>().unwrap();

        if num == 0 {
            if let Some((_,val)) = heap.pop() {
                write!(out,"{}\n", val * -1).unwrap();
            } else {
                write!(out,"0\n").unwrap()
            }
        } else {
            heap.push((num.abs() * -1, num * -1));
        }
        buf.clear();
    }
}