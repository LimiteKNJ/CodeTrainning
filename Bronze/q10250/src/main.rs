use io::Write;
use std::io;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let case = buf.trim().parse::<usize>().unwrap();

    for _ in 0..case {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let acm = buf.split_whitespace()
                    .map(|s|s.trim().parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

        let h = acm[0]; let w = acm[1];
        let c = acm[2];

        if c <= h*w {
            let mut ho = ((c / h) as f64).ceil() as usize + 1;
            let mut floor = c % h;
            if floor == 0 { floor = h; ho = ho - 1;}
            writeln!(out, "{}", floor*100 + ho).unwrap();

        } else {
            writeln!(out, "full").unwrap();
        }
    }
}