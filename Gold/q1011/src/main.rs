use std::io;
use io::Write;

        /*
            1   1
            2   1 1

            3   1 1 1
            4   1 2 1
            5   1 2 1 1
            6   1 2 2 1

            7   1 2 2 1 1
            8   1 2 2 2 1
            9   1 2 3 2 1 
            10  1 2 3 2 1 1 
            11  1 2 3 2 2 1 
            12  1 2 3 3 2 1   

            반복 개수의 증가 n(n+1)
            반복 개수 2n-1일 경우 : n^2보다 작음 / 2n일 경우 n^2보다 큼
        */

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
        let start = iter.next().unwrap().trim().parse::<u128>().unwrap();
        let target = iter.next().unwrap().trim().parse::<u128>().unwrap();       
        buf.clear();

        let distanse = target - start;
        let mut n = 0;

        loop {
            if distanse <= (n * (n+1)) {
                break;
            } n += 1;
        }
        
        if distanse <= (n * n) {
            write!(out, "{}\n", (n * 2) - 1).unwrap();

        } else {
            write!(out, "{}\n", n * 2).unwrap();
        } 
    }
}