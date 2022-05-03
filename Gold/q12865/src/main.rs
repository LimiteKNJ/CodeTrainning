use std::io::{Write, BufWriter, self};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let things_cnt = iter.next().unwrap().parse::<usize>().unwrap();
    let weight_max = iter.next().unwrap().parse::<usize>().unwrap();
    buf.clear();

    let mut things = vec![(0,0); things_cnt+1];
    for i in 0..things_cnt {
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let w = iter.next().unwrap().parse::<usize>().unwrap();
        let v = iter.next().unwrap().parse::<usize>().unwrap();
        things[i] = (w,v); buf.clear();
    }

    let mut max = 0;
    let mut dp = vec![vec![0; weight_max+1]; things_cnt+1];
    for i in 1..=things_cnt {
        for j in 1..=weight_max {
            if things[i-1].0 <= j {
                dp[i][j] = (things[i-1].1 + dp[i-1][j - things[i-1].0]).max(dp[i-1][j]);
            } else {
                dp[i][j] = dp[i-1][j];
            } max = max.max(dp[i][j]);
        }
    }

    write!(out, "{}", max).unwrap();
}