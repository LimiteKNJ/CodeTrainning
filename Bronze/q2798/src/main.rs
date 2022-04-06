use std::io;
use io::Write;

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let card_num = iter.next().unwrap().parse::<usize>().unwrap();
    let target_sum = iter.next().unwrap().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let mut cards = buf.split_whitespace()
                        .map(|s|s.trim().parse::<usize>().unwrap())
                        .collect::<Vec<_>>();

    let mut max = 0;
    for i in 0..card_num-2 {
        for j in i+1..card_num-1 {
            for k in j+1..card_num {
                let temp = cards[i] + cards[j] + cards[k]; 
                if temp > target_sum { continue; }
                if max < temp { max = temp; }
            }
        }
    }

    writeln!(out, "{}", max).unwrap();
}