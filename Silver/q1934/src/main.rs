use std::io::{Write, BufWriter, self};

fn main() {

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("no input");
    let case = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    for _ in 0..case {
        stdin.read_line(&mut buf).expect("no input");
        let mut iter = buf.split_whitespace();
        let a = iter.next().unwrap().trim().parse::<usize>().unwrap();
        let b = iter.next().unwrap().trim().parse::<usize>().unwrap();
        buf.clear();
        
        let lcmNum = lcm(a, b);
        write!(out, "{}\n", lcmNum).unwrap();
    }
}

fn gcb(x : usize, y : usize) -> usize {
    let mut _x = x;
    let mut _y = y;
    let mut _r: usize = 0;
    while(_y!=0){
        _r = _x%_y;
        _x = _y;
        _y = _r;
    }
    return _x;
}

fn lcm(x : usize, y : usize) -> usize {
    let mut _x = x;
    let mut _y = y;
    let res = ( _x * _y ) / gcb(_x, _y);
    return res;
}