use std::io::{Write, BufWriter, self};

fn fpow (num : u128, pow : u128) -> u128 {

    let div = 1000000007;
    if pow == 1 {
        return num % div;
    }

    let tmp  = fpow(num, pow/2);
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
    let num = buf.trim().parse::<u128>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let pow = buf.trim().parse::<u128>().unwrap();
    buf.clear();
    
    write!(out, "{}", fpow(num, pow) % 1000000007).unwrap();
}
