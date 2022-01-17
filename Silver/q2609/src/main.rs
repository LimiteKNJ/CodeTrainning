use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no input");
    let mut num: Vec<usize> = text.split_whitespace()
                            .map(|s|s.trim().parse().expect("Error"))
                            .collect::<Vec<_>>();

    let gcbNum = gcb(num[0], num[1]);
    let lcmNum = lcm(num[0], num[1]);
    
    print!("{} {}", gcbNum, lcmNum);
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