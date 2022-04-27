use std::io::{Write, BufWriter, self};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut dice = buf.split_whitespace()
                                    .map(|s|s.trim().parse::<usize>().unwrap())
                                    .collect::<Vec<_>>();
    let mut cnt = 0;
    dice.sort(); let mut max = dice[2];
    if dice[0] == dice[1] && dice[1] == dice[2] {
        cnt += 3;
    } else if dice[0] == dice[1] {
        cnt += 2; max = dice[0];
    } else if dice[0] == dice[2] {
        cnt += 2; max = dice[0];
    } else if dice[1] == dice[2] {
        cnt += 2; max = dice[1];
    } else {
        cnt += 1;
    }

    let money = match cnt {
        1 => (max * 100),
        2 => (max * 100 + 1000),
        3 => (max * 1000 + 10000),
        _ => 0
    }; write!(out, "{}",money).unwrap();
}