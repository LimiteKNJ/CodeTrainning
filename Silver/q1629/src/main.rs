use std::io::{Write, BufWriter, self};

fn fpow (num : u128, pow : u128, div : u128) -> u128 {

    if pow == 1 {
        return num % div;
    }

    let tmp  = fpow(num, pow/2, div);
    if pow % 2 == 0 {
        return tmp * tmp % div;
        // n * n 
    } else {
        return tmp * tmp * num % div;
        // n * n + 1
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let num = iter.next().unwrap().trim().parse::<u128>().unwrap();
    let pow = iter.next().unwrap().trim().parse::<u128>().unwrap();
    let div = iter.next().unwrap().trim().parse::<u128>().unwrap();
    write!(out, "{}", fpow(num, pow, div)).unwrap();
}
