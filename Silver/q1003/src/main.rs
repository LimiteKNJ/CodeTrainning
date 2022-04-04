use std::io;
use io::Write;



fn main() {
    
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let cnt = buf.trim().parse::<u128>().unwrap();

    for _ in 0..cnt {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<u128>().unwrap();

        let mut call_0 :u128 = 0; let mut call_1 : u128 = 0;
        let mut temp = 0;
        
        if num == 0 {
            call_0 = 1; call_1 = 0;

        } else if num == 1{
            call_0 = 0; call_1 = 1;

        } else {
            call_0 = 1; call_1 = 1;
            for _ in 2..num { 
                temp = call_0 + call_1;
                call_0 = call_1;
                call_1 = temp;
            }
        }  

        write!(out, "{} {}\n", call_0, call_1).unwrap();
    }
}