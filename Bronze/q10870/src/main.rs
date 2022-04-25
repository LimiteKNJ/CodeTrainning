use std::io::{Write, BufWriter, self};

fn fib(n : u128) -> u128 {
    let mut a:u128 = 1; 
    let mut b:u128 = 1; 
    let mut result:u128 = 0; 
    
    if n == 0 {
        return 0;
    } else if n <= 2 && n > 0 {
        return 1;
    } else {
        for _ in 2..n { 
            result = a + b;
            a = b;
            b = result; 
        }
    }  
    return result;
}

fn main() {
    
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<u128>().unwrap();

    write!(out, "{}", fib(num)).unwrap();
}