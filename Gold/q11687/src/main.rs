use std::io::{Write, Read, self};

fn main() {
    let stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let mut out = io::BufWriter::new(stdout.lock());
    
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut left : usize = 1; let mut right : usize = 10000000000;
    let mut mid = 0; let mut find = false; let mut result = 0;

    while left <= right {
        mid = (left + right) / 2;
        let mut cnt = 0;
        let mut iter = 5;
        while iter <= mid {
            cnt += mid /iter;
            iter *= 5;
        }

        if n < cnt {
            right = mid - 1;

        } else if n > cnt {
            left = mid + 1;
        
        } else {
            find = true;
            result = mid;
            break;
        }
    }
    
    if find {
        /* Error correction */
        writeln!(out, "{}", (result/5)*5).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}