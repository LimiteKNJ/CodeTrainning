use std::io::{Write, BufWriter, self};
fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let case = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    for _ in 0..case {
        stdin.read_line(&mut buf).unwrap();
        let cal_num = buf.trim().parse::<usize>().unwrap();
        buf.clear();

        let mut cal = Vec::new();
        for _ in 0..cal_num {
            stdin.read_line(&mut buf).unwrap();
            let mut iter = buf.split_whitespace();
            let cal_name = iter.next().unwrap().trim().to_string(); 
            let cal_al = iter.next().unwrap().trim().parse::<usize>().unwrap(); 
            cal.push((cal_name, cal_al));
            buf.clear();

        } cal.sort_by(|(a,b),(c,d)|d.cmp(&b));
        write!(out, "{}\n", cal.first().unwrap().0).unwrap();
    }
}
