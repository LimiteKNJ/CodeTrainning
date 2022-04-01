use std::io;
use io::Write;

fn fib(n : u128) -> u128 {
    let mut a:u128 = 1; 
    let mut b:u128 = 1; 
    let mut result:u128 = 0; 
    
    if n <= 2 {
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
    
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let num = buf.trim().parse::<u128>().unwrap();

    write!(out, "{}", fib(num)).unwrap();
}