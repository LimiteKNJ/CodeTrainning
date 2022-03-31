use std::io;
use io::Write;

// 알고리즘 이해 더 필요할 듯...

fn main() {
    let mut buf = String::new();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    std::io::stdin().read_line(&mut buf).unwrap();
    let sugar = buf.trim().parse::<usize>().unwrap();

    //    4  5  6   7  8  9  10  11  12  13  14 
    // 3 -1  0  2  -1  1  3   0   2   4   1   3
    // 5 -1  1  0  -1  1  0   2   1   0   2   1

    if sugar < 3 || sugar == 4 || sugar == 7 {
        writeln!(out,"-1").unwrap();

    } else if sugar % 5 == 0 {
        writeln!(out,"{}", sugar / 5).unwrap();

    } else if sugar % 5 == 1 || sugar % 5 == 3 {
        writeln!(out,"{}", (sugar / 5)+1).unwrap();
        
    } else if sugar % 5 == 2 || sugar % 5 == 4 {
        writeln!(out,"{}", (sugar / 5)+2).unwrap();
        
    } else { panic!("UNKNOWN!")}
}