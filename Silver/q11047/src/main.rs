use std::io;
use io::Write;

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let coins = iter.next().unwrap().parse::<usize>().unwrap();
    let mut money = iter.next().unwrap().parse::<usize>().unwrap();
    buf.clear();

    let mut coin_list : Vec<usize> = Vec::new();
    for _ in 0..coins {
        stdin.read_line(&mut buf).unwrap();
        coin_list.push(buf.trim().parse::<usize>().unwrap());
        buf.clear();
    } coin_list.sort_by(|a,b|b.cmp(&a));

    let mut counts = 0;
    for i in coin_list {
        counts += money / i;
        money = money % i;
    } write!(out, "{}", counts).unwrap();
}