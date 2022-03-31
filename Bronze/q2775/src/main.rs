use std::io;
use io::Write;

fn combine(k :u128, n :u128) -> u128 {
    if n == 0 {
        return k;
    } else {
        let mut num = 1; let mut del = 1;
        let mut m = 0;
        while m < n+1 {
            num *= k + m;
            m += 1;
            del *= m;
            
        } return num / del;
    }
}

fn main() {
    let mut buf = String::new();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    std::io::stdin().read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();

    for _ in 0..num {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut stair = buf.trim().parse::<u128>().unwrap();
    
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut ho = buf.trim().parse::<u128>().unwrap();

        writeln!(out, "{}", combine(ho, stair));
    }
}